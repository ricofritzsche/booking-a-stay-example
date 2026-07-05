use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_stay_starting_in_the_past() {
    let mut request = valid_request();
    request.stay = Stay {
        check_in: date(2026, 7, 4),
        check_out: date(2026, 7, 6),
    };

    assert_eq!(
        decide(
            &request,
            &valid_context(),
            reservation_id(),
            current_booking_time()
        ),
        Err(BookingRejected::StayStartsInPast)
    );
}

#[test]
fn allows_stay_starting_today() {
    let mut request = valid_request();
    request.stay = Stay {
        check_in: date(2026, 7, 5),
        check_out: date(2026, 7, 7),
    };

    let result = decide(
        &request,
        &valid_context(),
        reservation_id(),
        current_booking_time(),
    );

    assert!(result.is_ok());
}

#[test]
fn allows_stay_starting_in_the_future() {
    let mut request = valid_request();
    request.stay = Stay {
        check_in: date(2026, 7, 6),
        check_out: date(2026, 7, 8),
    };

    let result = decide(
        &request,
        &valid_context(),
        reservation_id(),
        current_booking_time(),
    );

    assert!(result.is_ok());
}
