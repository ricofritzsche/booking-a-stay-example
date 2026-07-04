use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_missing_listing() {
    let mut context = valid_context();
    context.listing = None;

    assert_eq!(
        decide(&valid_request(), &context, confirmed_at()),
        Err(BookingRejected::ListingNotFound)
    );
}

#[test]
fn rejects_disabled_listing() {
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .booking_status = ListingBookingStatus::Disabled;

    assert_eq!(
        decide(&valid_request(), &context, confirmed_at()),
        Err(BookingRejected::ListingDisabled)
    );
}
