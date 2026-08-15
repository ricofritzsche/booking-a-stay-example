using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.CoreTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.CoreTests;

public sealed class DateRangeTests
{
    [Fact]
    public void RejectsInvalidDateRange()
    {
        var original = ValidRequest();
        var request = original with { Stay = original.Stay with { CheckOut = original.Stay.CheckIn } };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.InvalidDateRange),
            Core.BookStay(request, ValidContext(), ReservationId(), ConfirmedAt()));
    }
}
