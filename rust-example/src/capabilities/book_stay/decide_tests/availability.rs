use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_when_an_unavailable_night_overlaps_the_stay() {
    let mut context = valid_context();
    context.unavailable_nights = vec![date(2026, 7, 2)];

    assert_eq!(
        decide(&valid_request(), &context, confirmed_at()),
        Err(BookingRejected::ListingUnavailable)
    );
}

#[test]
fn does_not_reject_when_the_only_unavailable_night_is_the_check_out_date() {
    let request = valid_request();
    let mut context = valid_context();
    context.unavailable_nights = vec![request.stay.check_out];

    let result = decide(&request, &context, confirmed_at());

    assert!(result.is_ok());
}
