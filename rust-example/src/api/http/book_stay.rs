//! HTTP adapter for the Book Stay capability.

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::application_state::AppState;
use crate::capabilities::book_stay::process::{
    BookStayResponse, ProcessBookStayError, process as process_book_stay,
};
use crate::capabilities::book_stay::request::{BookStay, Stay};
use crate::capabilities::book_stay::result::BookingRejected;

#[derive(Debug, Deserialize)]
pub struct BookStayRequestBody {
    pub guest_id: Uuid,
    pub listing_id: Uuid,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub guest_count: u32,
}

#[derive(Debug, Serialize)]
pub struct BookStayConfirmedBody {
    pub reservation_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    pub message: &'static str,
}

pub async fn handle(
    State(state): State<AppState>,
    Json(body): Json<BookStayRequestBody>,
) -> Response {
    let request = BookStay {
        guest_id: body.guest_id,
        listing_id: body.listing_id,
        stay: Stay {
            check_in: body.check_in,
            check_out: body.check_out,
        },
        guest_count: body.guest_count,
    };

    match process_book_stay(request, &state).await {
        Ok(BookStayResponse::Confirmed { reservation_id }) => (
            StatusCode::CREATED,
            Json(BookStayConfirmedBody { reservation_id }),
        )
            .into_response(),
        Ok(BookStayResponse::Rejected(rejection)) => rejection_response(rejection),
        Err(error) => technical_error_response(error),
    }
}

fn rejection_response(rejection: BookingRejected) -> Response {
    let (status, code, message) = rejection_http_mapping(rejection);

    (status, Json(ApiErrorBody { code, message })).into_response()
}

fn technical_error_response(error: ProcessBookStayError) -> Response {
    error!(
        error = %error,
        "book stay request failed with a technical error",
    );

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiErrorBody {
            code: "internal_error",
            message: "an internal error occurred",
        }),
    )
        .into_response()
}

fn rejection_http_mapping(rejection: BookingRejected) -> (StatusCode, &'static str, &'static str) {
    match rejection {
        BookingRejected::InvalidDateRange => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_date_range",
            "check-out must be after check-in",
        ),
        BookingRejected::InvalidGuestCount => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_guest_count",
            "guest count must be greater than zero",
        ),
        BookingRejected::StayStartsInPast => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "stay_starts_in_past",
            "check-in date must not be in the past",
        ),
        BookingRejected::GuestNotFound => {
            (StatusCode::NOT_FOUND, "guest_not_found", "guest not found")
        }
        BookingRejected::GuestBlocked => (
            StatusCode::CONFLICT,
            "guest_blocked",
            "guest is blocked from booking",
        ),
        BookingRejected::ListingNotFound => (
            StatusCode::NOT_FOUND,
            "listing_not_found",
            "listing not found",
        ),
        BookingRejected::ListingDisabled => (
            StatusCode::CONFLICT,
            "listing_disabled",
            "listing is disabled for booking",
        ),
        BookingRejected::TooManyGuests => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "too_many_guests",
            "guest count exceeds listing capacity",
        ),
        BookingRejected::StayTooShort => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "stay_too_short",
            "stay is shorter than the listing minimum",
        ),
        BookingRejected::StayTooLong => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "stay_too_long",
            "stay is longer than the listing maximum",
        ),
        BookingRejected::ListingUnavailable => (
            StatusCode::CONFLICT,
            "listing_unavailable",
            "listing is unavailable for the requested dates",
        ),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::{BookingRejected, rejection_http_mapping};

    #[test]
    fn maps_not_found_rejections_to_404() {
        assert_eq!(
            rejection_http_mapping(BookingRejected::GuestNotFound),
            (StatusCode::NOT_FOUND, "guest_not_found", "guest not found")
        );
        assert_eq!(
            rejection_http_mapping(BookingRejected::ListingNotFound),
            (
                StatusCode::NOT_FOUND,
                "listing_not_found",
                "listing not found"
            )
        );
    }

    #[test]
    fn maps_conflict_rejections_to_409() {
        assert_eq!(
            rejection_http_mapping(BookingRejected::ListingUnavailable),
            (
                StatusCode::CONFLICT,
                "listing_unavailable",
                "listing is unavailable for the requested dates"
            )
        );
    }

    #[test]
    fn maps_validation_rejections_to_422() {
        assert_eq!(
            rejection_http_mapping(BookingRejected::InvalidDateRange),
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_date_range",
                "check-out must be after check-in"
            )
        );
        assert_eq!(
            rejection_http_mapping(BookingRejected::StayStartsInPast),
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                "stay_starts_in_past",
                "check-in date must not be in the past"
            )
        );
    }
}
