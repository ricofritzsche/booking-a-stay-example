using BookingAStay.Providers;
using Npgsql;

namespace BookingAStay.Capabilities.BookStay;

public abstract record BookStayResponse
{
    private BookStayResponse()
    {
    }

    public sealed record Confirmed(Guid ReservationId) : BookStayResponse;

    public sealed record Rejected(BookingRejection Reason) : BookStayResponse;
}

public static class Processor
{
    public static async Task<BookStayResponse> ProcessBookStay(
        BookStayRequest request,
        NpgsqlDataSource dataSource,
        ProviderBundle providers,
        CancellationToken cancellationToken = default)
    {
        await using var connection = await dataSource.OpenConnectionAsync(cancellationToken);
        await using var transaction = await connection.BeginTransactionAsync(cancellationToken);

        try
        {
            var loadedState = await Sql.LoadBookingState(connection, transaction, request, cancellationToken);
            var context = loadedState.ToContext();
            var reservationId = providers.Ids.NewId();
            var now = providers.Clock.UtcNow();

            var outcome = Decider.Decide(request, context, reservationId, now);

            if (outcome is BookingOutcome.Rejected rejected)
            {
                await transaction.RollbackAsync(cancellationToken);
                return new BookStayResponse.Rejected(rejected.Reason);
            }

            var confirmed = ((BookingOutcome.Confirmed)outcome).Reservation;

            try
            {
                await Sql.RecordReservationConfirmed(connection, transaction, confirmed, cancellationToken);
            }
            catch (ListingUnavailableException)
            {
                await transaction.RollbackAsync(cancellationToken);
                return new BookStayResponse.Rejected(BookingRejection.ListingUnavailable);
            }

            await transaction.CommitAsync(cancellationToken);
            return new BookStayResponse.Confirmed(confirmed.ReservationId);
        }
        catch
        {
            if (transaction.Connection is not null)
            {
                await transaction.RollbackAsync(cancellationToken);
            }

            throw;
        }
    }
}
