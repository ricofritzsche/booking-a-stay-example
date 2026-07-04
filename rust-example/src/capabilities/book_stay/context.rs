use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuestBookingEligibility {
    Eligible,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListingBookingStatus {
    Bookable,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListingBookingSettings {
    pub booking_status: ListingBookingStatus,
    pub max_guests: u32,
    pub min_nights: u32,
    pub max_nights: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookingContext {
    pub guest: Option<GuestBookingEligibility>,
    pub listing: Option<ListingBookingSettings>,
    pub unavailable_nights: Vec<NaiveDate>,
}
