namespace BookingAStay.Capabilities.BookStay;

public enum GuestBookingEligibility
{
    Eligible,
    Blocked,
}

public enum ListingBookingStatus
{
    Bookable,
    Disabled,
}

public sealed record ListingBookingSettings(
    ListingBookingStatus BookingStatus,
    int MaxGuests,
    int MinNights,
    int? MaxNights);

// Input value for the BookStay Functional Core.
public sealed record BookingContext(
    GuestBookingEligibility? Guest,
    ListingBookingSettings? Listing,
    IReadOnlyList<DateOnly> UnavailableNights);
