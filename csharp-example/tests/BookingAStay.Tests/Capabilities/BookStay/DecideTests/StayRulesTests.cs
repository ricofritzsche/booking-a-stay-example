using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.DecideTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.DecideTests;

public sealed class StayRulesTests
{
    [Fact]
    public void RejectsTooManyGuests()
    {
        var request = ValidRequest() with { GuestCount = 5 };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.TooManyGuests),
            Decider.Decide(request, ValidContext(), ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsStayShorterThanMinNights()
    {
        var request = ValidRequest() with { Stay = Stay(1, 2) };
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { MinNights = 2 } };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.StayTooShort),
            Decider.Decide(request, context, ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsStayLongerThanMaxNights()
    {
        var request = ValidRequest() with { Stay = Stay(1, 5) };
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { MaxNights = 3 } };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.StayTooLong),
            Decider.Decide(request, context, ReservationId(), ConfirmedAt()));
    }
}
