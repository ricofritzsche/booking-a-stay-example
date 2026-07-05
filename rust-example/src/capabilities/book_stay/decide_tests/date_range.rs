use super::super::result::BookingRejected;
use super::*;

#[test]
fn rejects_invalid_date_range() {
    let mut request = valid_request();
    request.stay.check_out = request.stay.check_in;

    assert_eq!(
        decide(&request, &valid_context(), reservation_id(), confirmed_at()),
        Err(BookingRejected::InvalidDateRange)
    );
}
