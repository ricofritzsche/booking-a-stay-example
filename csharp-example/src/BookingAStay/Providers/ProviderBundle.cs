namespace BookingAStay.Providers;

// Groups the Providers used by the Imperative Shell.
public sealed record ProviderBundle(Clock Clock, IdGenerator Ids)
{
    public ProviderBundle()
        : this(new Clock(), new IdGenerator())
    {
    }
}
