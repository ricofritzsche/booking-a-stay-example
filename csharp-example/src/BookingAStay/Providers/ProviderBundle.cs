namespace BookingAStay.Providers;

public sealed record ProviderBundle(Clock Clock, IdGenerator Ids)
{
    public ProviderBundle()
        : this(new Clock(), new IdGenerator())
    {
    }
}
