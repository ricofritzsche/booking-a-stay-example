using BookingAStay.Providers;
using Npgsql;

namespace BookingAStay.Capabilities.BookStay;

// Public response returned by the BookStay RPU.
public abstract record BookStayResponse
{
    private BookStayResponse()
    {
    }

    public sealed record Confirmed(Guid ReservationId) : BookStayResponse;

    public sealed record Rejected(BookingRejection Reason) : BookStayResponse;
}

// Contains the Imperative Shell for the BookStay RPU.
public static class Processor
{
    // RPU entry point and Imperative Shell.
    // Strict IOSP: hybrid because it combines project-local function calls
    // with transaction and outcome-handling logic.
    public static async Task<BookStayResponse> ProcessAsync(
        BookStayRequest request,
        NpgsqlDataSource dataSource,
        ProviderBundle providers,
        CancellationToken cancellationToken = default)
    {
        // Imperative Shell: owns the connection and transaction lifecycle.
        await using var connection = await dataSource.OpenConnectionAsync(cancellationToken);
        await using var transaction = await connection.BeginTransactionAsync(cancellationToken);

        try
        {
            // Imperative Shell: loads state and obtains external values before calling the core.
            var loadedState = await StateAccess.LoadBookingState(connection, transaction, request, cancellationToken);
            var context = loadedState.ToContext();
            var reservationId = providers.Ids.NewId();
            var now = providers.Clock.UtcNow();

            // Functional Core: explicit values in, outcome out, no I/O.
            var outcome = Core.BookStay(request, context, reservationId, now);

            // Imperative Shell: maps the outcome to persistence and the public response.
            if (outcome is BookingOutcome.Rejected rejected)
            {
                await transaction.RollbackAsync(cancellationToken);
                return new BookStayResponse.Rejected(rejected.Reason);
            }

            var confirmed = ((BookingOutcome.Confirmed)outcome).Reservation;

            try
            {
                await StateAccess.RecordReservationConfirmed(connection, transaction, confirmed, cancellationToken);
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
