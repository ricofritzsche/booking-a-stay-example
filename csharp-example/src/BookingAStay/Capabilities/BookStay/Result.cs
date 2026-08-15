namespace BookingAStay.Capabilities.BookStay;

// Accepted outcome data produced by the Functional Core.
public sealed record ReservationConfirmed(
    Guid ReservationId,
    Guid GuestId,
    Guid ListingId,
    Stay Stay,
    int GuestCount,
    DateTimeOffset ConfirmedAt,
    int MaxGuestsAtConfirmation,
    int MinNightsAtConfirmation,
    int? MaxNightsAtConfirmation);

// Rejection values produced by the Functional Core.
public enum BookingRejection
{
    InvalidDateRange,
    InvalidGuestCount,
    StayStartsInPast,
    GuestNotFound,
    GuestBlocked,
    ListingNotFound,
    ListingDisabled,
    TooManyGuests,
    StayTooShort,
    StayTooLong,
    ListingUnavailable,
}

// Output value returned by the BookStay Functional Core.
public abstract record BookingOutcome
{
    private BookingOutcome()
    {
    }

    public sealed record Confirmed(ReservationConfirmed Reservation) : BookingOutcome;

    public sealed record Rejected(BookingRejection Reason) : BookingOutcome;
}
