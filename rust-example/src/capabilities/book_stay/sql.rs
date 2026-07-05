use chrono::NaiveDate;
use sqlx::{Postgres, Row, Transaction};

use super::context::{
    BookingContext, GuestBookingEligibility, ListingBookingSettings, ListingBookingStatus,
};
use super::request::BookStay;
use super::result::ReservationConfirmed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedBookingState {
    pub guest: Option<GuestBookingEligibility>,
    pub listing: Option<ListingBookingSettings>,
    pub unavailable_nights: Vec<NaiveDate>,
}

impl LoadedBookingState {
    pub fn into_context(self) -> BookingContext {
        BookingContext {
            guest: self.guest,
            listing: self.listing,
            unavailable_nights: self.unavailable_nights,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadBookingStateError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("invalid stored value: {0}")]
    InvalidStoredValue(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RecordReservationError {
    #[error("database error: {0}")]
    Database(sqlx::Error),

    #[error("invalid stored value: {0}")]
    InvalidStoredValue(String),

    #[error("listing unavailable")]
    ListingUnavailable,
}

pub async fn load_booking_state(
    tx: &mut Transaction<'_, Postgres>,
    request: &BookStay,
) -> Result<LoadedBookingState, LoadBookingStateError> {
    let guest = load_guest_eligibility(tx, request).await?;
    let listing = load_listing_booking_settings(tx, request).await?;
    let unavailable_nights = load_unavailable_nights(tx, request).await?;

    Ok(LoadedBookingState {
        guest,
        listing,
        unavailable_nights,
    })
}

async fn load_guest_eligibility(
    tx: &mut Transaction<'_, Postgres>,
    request: &BookStay,
) -> Result<Option<GuestBookingEligibility>, LoadBookingStateError> {
    let row = sqlx::query(
        r#"
        SELECT booking_eligibility
        FROM guests
        WHERE id = $1
        FOR SHARE
        "#,
    )
    .bind(request.guest_id)
    .fetch_optional(&mut **tx)
    .await?;

    row.map(|row| {
        let value: String = row.get("booking_eligibility");
        match value.as_str() {
            "eligible" => Ok(GuestBookingEligibility::Eligible),
            "blocked" => Ok(GuestBookingEligibility::Blocked),
            _ => Err(LoadBookingStateError::InvalidStoredValue(format!(
                "unknown guest booking eligibility: {value}"
            ))),
        }
    })
    .transpose()
}

async fn load_listing_booking_settings(
    tx: &mut Transaction<'_, Postgres>,
    request: &BookStay,
) -> Result<Option<ListingBookingSettings>, LoadBookingStateError> {
    let row = sqlx::query(
        r#"
        SELECT booking_status, max_guests, min_nights, max_nights
        FROM listings
        WHERE id = $1
        FOR SHARE
        "#,
    )
    .bind(request.listing_id)
    .fetch_optional(&mut **tx)
    .await?;

    row.map(|row| {
        let booking_status: String = row.get("booking_status");
        let max_guests: i32 = row.get("max_guests");
        let min_nights: i32 = row.get("min_nights");
        let max_nights: Option<i32> = row.get("max_nights");

        Ok(ListingBookingSettings {
            booking_status: map_listing_booking_status(&booking_status)?,
            max_guests: i32_to_u32("listings.max_guests", max_guests)?,
            min_nights: i32_to_u32("listings.min_nights", min_nights)?,
            max_nights: max_nights
                .map(|value| i32_to_u32("listings.max_nights", value))
                .transpose()?,
        })
    })
    .transpose()
}

async fn load_unavailable_nights(
    tx: &mut Transaction<'_, Postgres>,
    request: &BookStay,
) -> Result<Vec<NaiveDate>, LoadBookingStateError> {
    let rows = sqlx::query(
        r#"
        SELECT night
        FROM listing_unavailable_nights
        WHERE listing_id = $1
          AND night >= $2
          AND night < $3
        ORDER BY night
        "#,
    )
    .bind(request.listing_id)
    .bind(request.stay.check_in)
    .bind(request.stay.check_out)
    .fetch_all(&mut **tx)
    .await?;

    Ok(rows.into_iter().map(|row| row.get("night")).collect())
}

pub async fn record_reservation_confirmed(
    tx: &mut Transaction<'_, Postgres>,
    confirmed: &ReservationConfirmed,
) -> Result<(), RecordReservationError> {
    let guest_count = i32::try_from(confirmed.guest_count).map_err(|_| {
        RecordReservationError::InvalidStoredValue(
            "guest_count exceeds PostgreSQL INTEGER".to_owned(),
        )
    })?;
    let max_guests_at_confirmation = u32_to_i32(
        "max_guests_at_confirmation",
        confirmed.max_guests_at_confirmation,
    )?;
    let min_nights_at_confirmation = u32_to_i32(
        "min_nights_at_confirmation",
        confirmed.min_nights_at_confirmation,
    )?;
    let max_nights_at_confirmation = confirmed
        .max_nights_at_confirmation
        .map(|value| u32_to_i32("max_nights_at_confirmation", value))
        .transpose()?;

    sqlx::query(
        r#"
        INSERT INTO reservations (
            id,
            listing_id,
            guest_id,
            check_in,
            check_out,
            guest_count,
            status,
            confirmed_at,
            max_guests_at_confirmation,
            min_nights_at_confirmation,
            max_nights_at_confirmation
        )
        VALUES ($1, $2, $3, $4, $5, $6, 'confirmed', $7, $8, $9, $10)
        "#,
    )
    .bind(confirmed.reservation_id)
    .bind(confirmed.listing_id)
    .bind(confirmed.guest_id)
    .bind(confirmed.stay.check_in)
    .bind(confirmed.stay.check_out)
    .bind(guest_count)
    .bind(confirmed.confirmed_at)
    .bind(max_guests_at_confirmation)
    .bind(min_nights_at_confirmation)
    .bind(max_nights_at_confirmation)
    .execute(&mut **tx)
    .await
    .map_err(RecordReservationError::Database)?;

    let mut night = confirmed.stay.check_in;

    while night < confirmed.stay.check_out {
        sqlx::query(
            r#"
            INSERT INTO listing_unavailable_nights (
                listing_id,
                night,
                unavailability_type,
                reservation_id
            )
            VALUES ($1, $2, 'reservation', $3)
            "#,
        )
        .bind(confirmed.listing_id)
        .bind(night)
        .bind(confirmed.reservation_id)
        .execute(&mut **tx)
        .await
        .map_err(|error| {
            if is_unique_violation(&error) {
                RecordReservationError::ListingUnavailable
            } else {
                RecordReservationError::Database(error)
            }
        })?;

        night = night
            .succ_opt()
            .expect("date range should not overflow after validation");
    }

    Ok(())
}

fn map_listing_booking_status(value: &str) -> Result<ListingBookingStatus, LoadBookingStateError> {
    match value {
        "bookable" => Ok(ListingBookingStatus::Bookable),
        "disabled" => Ok(ListingBookingStatus::Disabled),
        _ => Err(LoadBookingStateError::InvalidStoredValue(format!(
            "unknown listing booking status: {value}"
        ))),
    }
}

fn i32_to_u32(field: &str, value: i32) -> Result<u32, LoadBookingStateError> {
    u32::try_from(value).map_err(|_| {
        LoadBookingStateError::InvalidStoredValue(format!(
            "{field} must be representable as u32: {value}"
        ))
    })
}

fn u32_to_i32(field: &str, value: u32) -> Result<i32, RecordReservationError> {
    i32::try_from(value).map_err(|_| {
        RecordReservationError::InvalidStoredValue(format!(
            "{field} exceeds PostgreSQL INTEGER: {value}"
        ))
    })
}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|database_error| database_error.code())
        .is_some_and(|code| code == "23505")
}
