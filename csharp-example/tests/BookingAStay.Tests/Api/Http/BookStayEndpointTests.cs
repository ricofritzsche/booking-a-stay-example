using BookingAStay.Api.Http;
using BookingAStay.Capabilities.BookStay;

namespace BookingAStay.Tests.Api.Http;

public sealed class BookStayEndpointTests
{
    [Fact]
    public void MapsNotFoundRejectionsTo404()
    {
        Assert.Equal(
            (404, "guest_not_found", "guest not found"),
            BookStayEndpoint.RejectionHttpMapping(BookingRejection.GuestNotFound));
        Assert.Equal(
            (404, "listing_not_found", "listing not found"),
            BookStayEndpoint.RejectionHttpMapping(BookingRejection.ListingNotFound));
    }

    [Fact]
    public void MapsConflictRejectionsTo409()
    {
        Assert.Equal(
            (409, "listing_unavailable", "listing is unavailable for the requested dates"),
            BookStayEndpoint.RejectionHttpMapping(BookingRejection.ListingUnavailable));
    }

    [Fact]
    public void MapsValidationRejectionsTo422()
    {
        Assert.Equal(
            (422, "invalid_date_range", "check-out must be after check-in"),
            BookStayEndpoint.RejectionHttpMapping(BookingRejection.InvalidDateRange));
        Assert.Equal(
            (422, "stay_starts_in_past", "check-in date must not be in the past"),
            BookStayEndpoint.RejectionHttpMapping(BookingRejection.StayStartsInPast));
    }
}
