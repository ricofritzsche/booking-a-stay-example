using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.DecideTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.DecideTests;

public sealed class PastDateTests
{
    [Fact]
    public void RejectsStayStartingInThePast()
    {
        var request = ValidRequest() with { Stay = new Stay(Date(2026, 7, 4), Date(2026, 7, 6)) };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.StayStartsInPast),
            Decider.Decide(request, ValidContext(), ReservationId(), CurrentBookingTime()));
    }

    [Fact]
    public void AllowsStayStartingToday()
    {
        var request = ValidRequest() with { Stay = new Stay(Date(2026, 7, 5), Date(2026, 7, 7)) };

        var result = Decider.Decide(request, ValidContext(), ReservationId(), CurrentBookingTime());

        Assert.IsType<BookingOutcome.Confirmed>(result);
    }

    [Fact]
    public void AllowsStayStartingInTheFuture()
    {
        var request = ValidRequest() with { Stay = new Stay(Date(2026, 7, 6), Date(2026, 7, 8)) };

        var result = Decider.Decide(request, ValidContext(), ReservationId(), CurrentBookingTime());

        Assert.IsType<BookingOutcome.Confirmed>(result);
    }
}
