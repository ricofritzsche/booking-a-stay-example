use uuid::Uuid;

use crate::application_state::AppState;

use super::decide::decide;
use super::request::BookStay;
use super::result::BookingRejected;
use super::sql::{
    LoadBookingStateError, RecordReservationError, load_booking_state, record_reservation_confirmed,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookStayResponse {
    Confirmed { reservation_id: Uuid },
    Rejected(BookingRejected),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessBookStayError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("invalid stored value: {0}")]
    InvalidStoredValue(String),
}

pub async fn process(
    request: BookStay,
    state: &AppState,
) -> Result<BookStayResponse, ProcessBookStayError> {
    let mut tx = state.pool.begin().await?;

    let loaded_state = match load_booking_state(&mut tx, &request).await {
        Ok(loaded_state) => loaded_state,
        Err(error) => {
            tx.rollback().await?;
            return Err(error.into());
        }
    };

    let context = loaded_state.into_context();
    let reservation_id = state.providers.ids.new_id();
    let now = state.providers.clock.now();

    let confirmed = match decide(&request, &context, reservation_id, now) {
        Ok(confirmed) => confirmed,
        Err(rejection) => {
            tx.rollback().await?;
            return Ok(BookStayResponse::Rejected(rejection));
        }
    };

    match record_reservation_confirmed(&mut tx, &confirmed).await {
        Ok(()) => {
            tx.commit().await?;
            Ok(BookStayResponse::Confirmed {
                reservation_id: confirmed.reservation_id,
            })
        }
        Err(RecordReservationError::ListingUnavailable) => {
            tx.rollback().await?;
            Ok(BookStayResponse::Rejected(
                BookingRejected::ListingUnavailable,
            ))
        }
        Err(RecordReservationError::Database(error)) => {
            tx.rollback().await?;
            Err(ProcessBookStayError::Database(error))
        }
        Err(RecordReservationError::InvalidStoredValue(detail)) => {
            tx.rollback().await?;
            Err(ProcessBookStayError::InvalidStoredValue(detail))
        }
    }
}

impl From<LoadBookingStateError> for ProcessBookStayError {
    fn from(error: LoadBookingStateError) -> Self {
        match error {
            LoadBookingStateError::Database(error) => Self::Database(error),
            LoadBookingStateError::InvalidStoredValue(detail) => Self::InvalidStoredValue(detail),
        }
    }
}
