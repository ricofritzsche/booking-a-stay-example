namespace BookingAStay.Capabilities.BookStay;

// Domain Request accepted by the BookStay RPU.
public sealed record BookStayRequest(Guid GuestId, Guid ListingId, Stay Stay, int GuestCount);

public sealed record Stay(DateOnly CheckIn, DateOnly CheckOut);
