use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_too_many_guests() {
    let mut request = valid_request();
    request.guest_count = 5;

    assert_eq!(
        decide(&request, &valid_context(), confirmed_at()),
        Err(BookingRejected::TooManyGuests)
    );
}

#[test]
fn rejects_stay_shorter_than_min_nights() {
    let mut request = valid_request();
    request.stay = stay(1, 2);
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .min_nights = 2;

    assert_eq!(
        decide(&request, &context, confirmed_at()),
        Err(BookingRejected::StayTooShort)
    );
}

#[test]
fn rejects_stay_longer_than_max_nights() {
    let mut request = valid_request();
    request.stay = stay(1, 5);
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .max_nights = Some(3);

    assert_eq!(
        decide(&request, &context, confirmed_at()),
        Err(BookingRejected::StayTooLong)
    );
}
