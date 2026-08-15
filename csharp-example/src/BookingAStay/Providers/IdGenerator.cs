namespace BookingAStay.Providers;

// Provider that gives the Imperative Shell access to new identifiers.
public sealed class IdGenerator
{
    // Resource Operation: obtains a new identifier from the runtime environment.
    public Guid NewId()
    {
        return Guid.NewGuid();
    }
}
