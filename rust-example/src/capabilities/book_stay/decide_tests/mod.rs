use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use super::context::{
    BookingContext, GuestBookingEligibility, ListingBookingSettings, ListingBookingStatus,
};
use super::decide::decide;
use super::request::{BookStay, Stay};

mod availability;
mod confirmation;
mod date_range;
mod guest;
mod listing;
mod stay_rules;

fn valid_request() -> BookStay {
    BookStay {
        reservation_id: Uuid::from_u128(1),
        guest_id: Uuid::from_u128(2),
        listing_id: Uuid::from_u128(3),
        stay: stay(1, 4),
        guest_count: 2,
    }
}

fn valid_context() -> BookingContext {
    BookingContext {
        guest: Some(GuestBookingEligibility::Eligible),
        listing: Some(ListingBookingSettings {
            booking_status: ListingBookingStatus::Bookable,
            max_guests: 4,
            min_nights: 1,
            max_nights: Some(10),
        }),
        unavailable_nights: Vec::new(),
    }
}

fn stay(check_in_day: u32, check_out_day: u32) -> Stay {
    Stay {
        check_in: date(2026, 7, check_in_day),
        check_out: date(2026, 7, check_out_day),
    }
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture date must be valid")
}

fn confirmed_at() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-07-01T12:00:00Z")
        .expect("fixture timestamp must be valid")
        .with_timezone(&Utc)
}
