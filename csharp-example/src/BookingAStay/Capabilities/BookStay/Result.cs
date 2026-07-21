namespace BookingAStay.Capabilities.BookStay;

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

public abstract record BookingOutcome
{
    private BookingOutcome()
    {
    }

    public sealed record Confirmed(ReservationConfirmed Reservation) : BookingOutcome;

    public sealed record Rejected(BookingRejection Reason) : BookingOutcome;
}
