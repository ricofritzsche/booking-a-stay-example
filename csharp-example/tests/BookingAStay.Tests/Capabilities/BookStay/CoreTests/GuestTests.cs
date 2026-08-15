using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.CoreTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.CoreTests;

public sealed class GuestTests
{
    [Fact]
    public void RejectsGuestCountZero()
    {
        var request = ValidRequest() with { GuestCount = 0 };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.InvalidGuestCount),
            Core.BookStay(request, ValidContext(), ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsNegativeGuestCount()
    {
        var request = ValidRequest() with { GuestCount = -1 };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.InvalidGuestCount),
            Core.BookStay(request, ValidContext(), ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsMissingGuest()
    {
        var context = ValidContext() with { Guest = null };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.GuestNotFound),
            Core.BookStay(ValidRequest(), context, ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsBlockedGuest()
    {
        var context = ValidContext() with { Guest = GuestBookingEligibility.Blocked };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.GuestBlocked),
            Core.BookStay(ValidRequest(), context, ReservationId(), ConfirmedAt()));
    }
}
