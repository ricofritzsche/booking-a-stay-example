namespace BookingAStay.Capabilities.BookStay;

public static class Decider
{
    public static BookingOutcome Decide(
        BookStayRequest request,
        BookingContext context,
        Guid reservationId,
        DateTimeOffset now)
    {
        if (request.Stay.CheckIn >= request.Stay.CheckOut)
        {
            return new BookingOutcome.Rejected(BookingRejection.InvalidDateRange);
        }

        if (request.GuestCount <= 0)
        {
            return new BookingOutcome.Rejected(BookingRejection.InvalidGuestCount);
        }

        if (request.Stay.CheckIn < DateOnly.FromDateTime(now.UtcDateTime))
        {
            return new BookingOutcome.Rejected(BookingRejection.StayStartsInPast);
        }

        switch (context.Guest)
        {
            case null:
                return new BookingOutcome.Rejected(BookingRejection.GuestNotFound);
            case GuestBookingEligibility.Blocked:
                return new BookingOutcome.Rejected(BookingRejection.GuestBlocked);
            case GuestBookingEligibility.Eligible:
                break;
        }

        var listing = context.Listing;
        if (listing is null)
        {
            return new BookingOutcome.Rejected(BookingRejection.ListingNotFound);
        }

        var listingRejection = ValidateListing(request, listing);
        if (listingRejection is not null)
        {
            return new BookingOutcome.Rejected(listingRejection.Value);
        }

        if (context.UnavailableNights.Any(night => night >= request.Stay.CheckIn && night < request.Stay.CheckOut))
        {
            return new BookingOutcome.Rejected(BookingRejection.ListingUnavailable);
        }

        return new BookingOutcome.Confirmed(new ReservationConfirmed(
            reservationId,
            request.GuestId,
            request.ListingId,
            request.Stay,
            request.GuestCount,
            now,
            listing.MaxGuests,
            listing.MinNights,
            listing.MaxNights));
    }

    private static BookingRejection? ValidateListing(BookStayRequest request, ListingBookingSettings listing)
    {
        if (listing.BookingStatus == ListingBookingStatus.Disabled)
        {
            return BookingRejection.ListingDisabled;
        }

        if (request.GuestCount > listing.MaxGuests)
        {
            return BookingRejection.TooManyGuests;
        }

        var stayNights = request.Stay.CheckOut.DayNumber - request.Stay.CheckIn.DayNumber;

        if (stayNights < listing.MinNights)
        {
            return BookingRejection.StayTooShort;
        }

        if (listing.MaxNights is { } maxNights && stayNights > maxNights)
        {
            return BookingRejection.StayTooLong;
        }

        return null;
    }
}
