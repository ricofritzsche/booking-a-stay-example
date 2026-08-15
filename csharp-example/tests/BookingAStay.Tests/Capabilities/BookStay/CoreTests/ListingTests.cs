using BookingAStay.Capabilities.BookStay;
using static BookingAStay.Tests.Capabilities.BookStay.CoreTests.Fixtures;

namespace BookingAStay.Tests.Capabilities.BookStay.CoreTests;

public sealed class ListingTests
{
    [Fact]
    public void RejectsMissingListing()
    {
        var context = ValidContext() with { Listing = null };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.ListingNotFound),
            Core.BookStay(ValidRequest(), context, ReservationId(), ConfirmedAt()));
    }

    [Fact]
    public void RejectsDisabledListing()
    {
        var listing = ValidContext().Listing!;
        var context = ValidContext() with { Listing = listing with { BookingStatus = ListingBookingStatus.Disabled } };

        Assert.Equal(
            new BookingOutcome.Rejected(BookingRejection.ListingDisabled),
            Core.BookStay(ValidRequest(), context, ReservationId(), ConfirmedAt()));
    }
}
