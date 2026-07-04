use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookStay {
    pub reservation_id: Uuid,
    pub guest_id: Uuid,
    pub listing_id: Uuid,
    pub stay: Stay,
    pub guest_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stay {
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
}
