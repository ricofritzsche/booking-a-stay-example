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
        })
    );
}
