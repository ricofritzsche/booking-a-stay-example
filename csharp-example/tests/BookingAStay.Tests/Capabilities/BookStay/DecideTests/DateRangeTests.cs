using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.DecideTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.DecideTests;

public sealed class DateRangeTests
{
    [Fact]
    public void RejectsInvalidDateRange()
    {
        var original = ValidRequest();
        var request = original with { Stay = original.Stay with { CheckOut = original.Stay.CheckIn } };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.InvalidDateRange),
            Decider.Decide(request, ValidContext(), ReservationId(), ConfirmedAt()));
    }
}
