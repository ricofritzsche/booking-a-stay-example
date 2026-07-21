namespace BookingAStay.Providers;

public sealed record Providers(Clock Clock, IdGenerator Ids)
{
    public Providers()
        : this(new Clock(), new IdGenerator())
    {
    }
}
