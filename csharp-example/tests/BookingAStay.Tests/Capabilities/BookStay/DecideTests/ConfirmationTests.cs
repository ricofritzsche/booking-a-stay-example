using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.DecideTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.DecideTests;

public sealed class ConfirmationTests
{
    [Fact]
    public void ConfirmsReservationWhenAllConditionsAreValid()
    {
        var request = ValidRequest();
        var context = ValidContext();
        var reservationId = ReservationId();
        var confirmedAt = ConfirmedAt();

        var result = Decider.Decide(request, context, reservationId, confirmedAt);

        Assert.Equal(
            new BookingOutcome.Confirmed(new ReservationConfirmed(
                reservationId,
                request.GuestId,
                request.ListingId,
                request.Stay,
                request.GuestCount,
                confirmedAt,
                4,
                1,
                10)),
            result);
    }

    [Fact]
    public void ConfirmedReservationIncludesMaxGuestsAtConfirmation()
    {
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { MaxGuests = 5 } };

        var result = Assert.IsType<BookingOutcome.Confirmed>(
            Decider.Decide(ValidRequest(), context, ReservationId(), ConfirmedAt()));

        Assert.Equal(5, result.Reservation.MaxGuestsAtConfirmation);
    }

    [Fact]
    public void ConfirmedReservationIncludesMinNightsAtConfirmation()
    {
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { MinNights = 3 } };

        var result = Assert.IsType<BookingOutcome.Confirmed>(
            Decider.Decide(ValidRequest(), context, ReservationId(), ConfirmedAt()));

        Assert.Equal(3, result.Reservation.MinNightsAtConfirmation);
    }

    [Fact]
    public void ConfirmedReservationIncludesMaxNightsAtConfirmation()
    {
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { MaxNights = 6 } };

        var result = Assert.IsType<BookingOutcome.Confirmed>(
            Decider.Decide(ValidRequest(), context, ReservationId(), ConfirmedAt()));

        Assert.Equal(6, result.Reservation.MaxNightsAtConfirmation);
    }
}
