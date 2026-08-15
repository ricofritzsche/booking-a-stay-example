namespace BookingAStay.Providers;

// Provider that gives the Imperative Shell access to the current time.
public sealed class Clock
{
    // Resource Operation: reads time from the runtime environment.
    public DateTimeOffset UtcNow()
    {
        return DateTimeOffset.UtcNow;
    }
}
