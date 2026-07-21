namespace BookingAStay.Capabilities.BookStay;

public sealed record BookStayRequest(Guid GuestId, Guid ListingId, Stay Stay, int GuestCount);

public sealed record Stay(DateOnly CheckIn, DateOnly CheckOut);
