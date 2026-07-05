use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::context::{
    BookingContext, GuestBookingEligibility, ListingBookingSettings, ListingBookingStatus,
};
use super::request::BookStay;
use super::result::{BookingRejected, ReservationConfirmed};

pub fn decide(
    request: &BookStay,
    context: &BookingContext,
    reservation_id: Uuid,
    now: DateTime<Utc>,
) -> Result<ReservationConfirmed, BookingRejected> {
    if request.stay.check_in >= request.stay.check_out {
        return Err(BookingRejected::InvalidDateRange);
    }

    if request.guest_count == 0 {
        return Err(BookingRejected::InvalidGuestCount);
    }

    if request.stay.check_in < now.date_naive() {
        return Err(BookingRejected::StayStartsInPast);
    }

    match context.guest {
        None => return Err(BookingRejected::GuestNotFound),
        Some(GuestBookingEligibility::Blocked) => return Err(BookingRejected::GuestBlocked),
        Some(GuestBookingEligibility::Eligible) => {}
    }

    let listing = match &context.listing {
        None => return Err(BookingRejected::ListingNotFound),
        Some(listing) => listing,
    };

    validate_listing(request, listing)?;

    if context
        .unavailable_nights
        .iter()
        .any(|night| *night >= request.stay.check_in && *night < request.stay.check_out)
    {
        return Err(BookingRejected::ListingUnavailable);
    }

    Ok(ReservationConfirmed {
        reservation_id,
        guest_id: request.guest_id,
        listing_id: request.listing_id,
        stay: request.stay,
        guest_count: request.guest_count,
        confirmed_at: now,
    })
}

fn validate_listing(
    request: &BookStay,
    listing: &ListingBookingSettings,
) -> Result<(), BookingRejected> {
    if listing.booking_status == ListingBookingStatus::Disabled {
        return Err(BookingRejected::ListingDisabled);
    }

    if request.guest_count > listing.max_guests {
        return Err(BookingRejected::TooManyGuests);
    }

    let stay_nights = request
        .stay
        .check_out
        .signed_duration_since(request.stay.check_in)
        .num_days();
    let min_nights = i64::from(listing.min_nights);
    let max_nights = listing.max_nights.map(i64::from);

    if stay_nights < min_nights {
        return Err(BookingRejected::StayTooShort);
    }

    if max_nights.is_some_and(|max_nights| stay_nights > max_nights) {
        return Err(BookingRejected::StayTooLong);
    }

    Ok(())
}
