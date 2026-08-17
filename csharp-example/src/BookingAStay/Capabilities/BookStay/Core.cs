namespace BookingAStay.Capabilities.BookStay;

// Functional Core for the BookStay RPU.
public static class Core
{
    // Integration: composes pure Domain Operations and passes their values.
    public static BookingOutcome BookStay(
        BookStayRequest request,
        BookingContext context,
        Guid reservationId,
        DateTimeOffset now)
    {
        return ToOutcome(
            request,
            context,
            reservationId,
            now,
            FirstRejection(
                ValidateDateRange(request),
                ValidateGuestCount(request),
                ValidatePastCheckIn(request, now),
                ValidateGuest(context.Guest),
                ValidateListingExists(context.Listing),
                ValidateListing(request, context.Listing),
                ValidateAvailability(request, context.UnavailableNights)));
    }

    // Domain Operation: validates the requested date range.
    private static BookingRejection? ValidateDateRange(BookStayRequest request)
    {
        return request.Stay.CheckIn >= request.Stay.CheckOut
            ? BookingRejection.InvalidDateRange
            : null;
    }

    // Domain Operation: validates the requested guest count.
    private static BookingRejection? ValidateGuestCount(BookStayRequest request)
    {
        return request.GuestCount <= 0
            ? BookingRejection.InvalidGuestCount
            : null;
    }

    // Domain Operation: validates that check-in is not in the past.
    private static BookingRejection? ValidatePastCheckIn(BookStayRequest request, DateTimeOffset now)
    {
        return request.Stay.CheckIn < DateOnly.FromDateTime(now.UtcDateTime)
            ? BookingRejection.StayStartsInPast
            : null;
    }

    // Domain Operation: validates guest existence and eligibility.
    private static BookingRejection? ValidateGuest(GuestBookingEligibility? guest)
    {
        return guest switch
        {
            null => BookingRejection.GuestNotFound,
            GuestBookingEligibility.Blocked => BookingRejection.GuestBlocked,
            _ => null,
        };
    }

    // Domain Operation: validates listing existence.
    private static BookingRejection? ValidateListingExists(ListingBookingSettings? listing)
    {
        return listing is null
            ? BookingRejection.ListingNotFound
            : null;
    }

    // Domain Operation: validates listing booking rules.
    private static BookingRejection? ValidateListing(
        BookStayRequest request,
        ListingBookingSettings? listing)
    {
        if (listing is null)
        {
            return null;
        }

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

    // Domain Operation: validates listing availability for the requested stay.
    private static BookingRejection? ValidateAvailability(
        BookStayRequest request,
        IReadOnlyList<DateOnly> unavailableNights)
    {
        return unavailableNights.Any(
            night => night >= request.Stay.CheckIn && night < request.Stay.CheckOut)
            ? BookingRejection.ListingUnavailable
            : null;
    }

    // Domain Operation: preserves rejection precedence for eagerly evaluated validators.
    private static BookingRejection? FirstRejection(params BookingRejection?[] candidates)
    {
        foreach (var candidate in candidates)
        {
            if (candidate is not null)
            {
                return candidate;
            }
        }

        return null;
    }

    // Domain Operation: converts the selected rejection or confirmation data into the outcome.
    private static BookingOutcome ToOutcome(
        BookStayRequest request,
        BookingContext context,
        Guid reservationId,
        DateTimeOffset now,
        BookingRejection? rejection)
    {
        if (rejection is { } reason)
        {
            return new BookingOutcome.Rejected(reason);
        }

        if (context.Listing is not { } listing)
        {
            return new BookingOutcome.Rejected(BookingRejection.ListingNotFound);
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
}
