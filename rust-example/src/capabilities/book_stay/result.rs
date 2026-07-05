use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::request::Stay;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservationConfirmed {
    pub reservation_id: Uuid,
    pub guest_id: Uuid,
    pub listing_id: Uuid,
    pub stay: Stay,
    pub guest_count: u32,
    pub confirmed_at: DateTime<Utc>,
    pub max_guests_at_confirmation: u32,
    pub min_nights_at_confirmation: u32,
    pub max_nights_at_confirmation: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookingRejected {
    InvalidDateRange,
    InvalidGuestCount,
    StayStartsInPast,
    GuestNotFound,
    GuestBlocked,
    ListingNotFound,
    ListingDisabled,
    TooManyGuests,
    StayTooShort,
    StayTooLong,
    ListingUnavailable,
}
