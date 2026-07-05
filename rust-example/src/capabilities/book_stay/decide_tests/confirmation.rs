use super::super::result::ReservationConfirmed;
use super::*;

#[test]
fn confirms_reservation_when_all_conditions_are_valid() {
    let request = valid_request();
    let context = valid_context();
    let reservation_id = reservation_id();
    let confirmed_at = confirmed_at();

    let result = decide(&request, &context, reservation_id, confirmed_at);

    assert_eq!(
        result,
        Ok(ReservationConfirmed {
            reservation_id,
            guest_id: request.guest_id,
            listing_id: request.listing_id,
            stay: request.stay,
            guest_count: request.guest_count,
            confirmed_at,
            max_guests_at_confirmation: 4,
            min_nights_at_confirmation: 1,
            max_nights_at_confirmation: Some(10),
        })
    );
}

#[test]
fn confirmed_reservation_includes_max_guests_at_confirmation() {
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .max_guests = 5;

    let result = decide(&valid_request(), &context, reservation_id(), confirmed_at())
        .expect("reservation should be confirmed");

    assert_eq!(result.max_guests_at_confirmation, 5);
}

#[test]
fn confirmed_reservation_includes_min_nights_at_confirmation() {
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .min_nights = 3;

    let result = decide(&valid_request(), &context, reservation_id(), confirmed_at())
        .expect("reservation should be confirmed");

    assert_eq!(result.min_nights_at_confirmation, 3);
}

#[test]
fn confirmed_reservation_includes_max_nights_at_confirmation() {
    let mut context = valid_context();
    context
        .listing
        .as_mut()
        .expect("valid fixture has listing")
        .max_nights = Some(6);

    let result = decide(&valid_request(), &context, reservation_id(), confirmed_at())
        .expect("reservation should be confirmed");

    assert_eq!(result.max_nights_at_confirmation, Some(6));
}
