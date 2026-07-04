use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_guest_count_zero() {
    let mut request = valid_request();
    request.guest_count = 0;

    assert_eq!(
        decide(&request, &valid_context(), confirmed_at()),
        Err(BookingRejected::InvalidGuestCount)
    );
}

#[test]
fn rejects_missing_guest() {
    let mut context = valid_context();
    context.guest = None;

    assert_eq!(
        decide(&valid_request(), &context, confirmed_at()),
        Err(BookingRejected::GuestNotFound)
    );
}

#[test]
fn rejects_blocked_guest() {
    let mut context = valid_context();
    context.guest = Some(GuestBookingEligibility::Blocked);

    assert_eq!(
        decide(&valid_request(), &context, confirmed_at()),
        Err(BookingRejected::GuestBlocked)
    );
}
