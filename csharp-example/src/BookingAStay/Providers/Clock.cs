namespace BookingAStay.Providers;

public sealed class Clock
{
    public DateTimeOffset UtcNow()
    {
        return DateTimeOffset.UtcNow;
    }
}
