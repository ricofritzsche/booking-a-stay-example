using BookingAStay.Capabilities.BookStay;

namespace BookingAStay.Tests.Capabilities.BookStay.DecideTests;

internal static class Fixtures
{
    public static BookStayRequest ValidRequest()
    {
        return new BookStayRequest(
            GuidFromByte(2),
            GuidFromByte(3),
            Stay(1, 4),
            2);
    }

    public static Guid ReservationId()
    {
        return GuidFromByte(1);
    }

    public static BookingContext ValidContext()
    {
        return new BookingContext(
            GuestBookingEligibility.Eligible,
            new ListingBookingSettings(
                ListingBookingStatus.Bookable,
                4,
                1,
                10),
            []);
    }

    public static Stay Stay(int checkInDay, int checkOutDay)
    {
        return new Stay(Date(2026, 7, checkInDay), Date(2026, 7, checkOutDay));
    }

    public static DateOnly Date(int year, int month, int day)
    {
        return new DateOnly(year, month, day);
    }

    public static DateTimeOffset ConfirmedAt()
    {
        return DateTimeOffset.Parse("2026-07-01T12:00:00Z");
    }

    public static DateTimeOffset CurrentBookingTime()
    {
        return DateTimeOffset.Parse("2026-07-05T12:00:00Z");
    }

    public static Guid GuidFromByte(uint value)
    {
        Span<byte> bytes = stackalloc byte[16];
        bytes[15] = (byte)value;
        return new Guid(bytes, bigEndian: true);
    }
}
