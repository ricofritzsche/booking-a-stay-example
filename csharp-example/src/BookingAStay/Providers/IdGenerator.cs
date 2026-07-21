namespace BookingAStay.Providers;

public sealed class IdGenerator
{
    public Guid NewId()
    {
        return Guid.NewGuid();
    }
}
