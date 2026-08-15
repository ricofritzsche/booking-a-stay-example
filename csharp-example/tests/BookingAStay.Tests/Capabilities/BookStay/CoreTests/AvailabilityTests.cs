using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.CoreTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.CoreTests;

public sealed class AvailabilityTests
{
    [Fact]
    public void RejectsWhenAnUnavailableNightOverlapsTheStay()
    {
        var context = ValidContext() with { UnavailableNights = [Date(2026, 7, 2)] };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.ListingUnavailable),
            Core.BookStay(ValidRequest(), context, ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void DoesNotRejectWhenTheOnlyUnavailableNightIsTheCheckOutDate()
    {
        var request = ValidRequest();
        var context = ValidContext() with { UnavailableNights = [request.Stay.CheckOut] };

        var result = Core.BookStay(request, context, ReservationId(), ConfirmedAt());

        Assert.IsType<BookingOutcome.Confirmed>(result);
    }
}
