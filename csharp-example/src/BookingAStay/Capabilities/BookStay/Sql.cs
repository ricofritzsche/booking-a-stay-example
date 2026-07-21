using Dapper;
using Npgsql;

namespace BookingAStay.Capabilities.BookStay;

public sealed record LoadedBookingState(
    GuestBookingEligibility? Guest,
    ListingBookingSettings? Listing,
    IReadOnlyList<DateOnly> UnavailableNights)
{
    public BookingContext ToContext()
    {
        return new BookingContext(Guest, Listing, UnavailableNights);
    }
}

public sealed class InvalidStoredValueException(string message) : Exception(message);

public sealed class ListingUnavailableException : Exception
{
}

public static class Sql
{
    internal sealed record GuestRow(string BookingEligibility);

    internal sealed record ListingRow(
        string BookingStatus,
        int MaxGuests,
        int MinNights,
        int? MaxNights);

    internal sealed record UnavailableNightRow(DateOnly Night);

    public static async Task<LoadedBookingState> LoadBookingState(
        NpgsqlConnection connection,
        NpgsqlTransaction transaction,
        BookStayRequest request,
        CancellationToken cancellationToken = default)
    {
        var guest = await LoadGuestEligibility(connection, transaction, request, cancellationToken);
        var listing = await LoadListingBookingSettings(connection, transaction, request, cancellationToken);
        var unavailableNights = await LoadUnavailableNights(connection, transaction, request, cancellationToken);

        return new LoadedBookingState(guest, listing, unavailableNights);
    }

    public static async Task RecordReservationConfirmed(
        NpgsqlConnection connection,
        NpgsqlTransaction transaction,
        ReservationConfirmed confirmed,
        CancellationToken cancellationToken = default)
    {
        const string reservationSql = """
            INSERT INTO reservations (
                id,
                listing_id,
                guest_id,
                check_in,
                check_out,
                guest_count,
                status,
                confirmed_at,
                max_guests_at_confirmation,
                min_nights_at_confirmation,
                max_nights_at_confirmation
            )
            VALUES (@id, @listing_id, @guest_id, @check_in, @check_out, @guest_count, 'confirmed', @confirmed_at, @max_guests, @min_nights, @max_nights)
            """;

        await connection.ExecuteAsync(
            reservationSql,
            new
            {
                id = confirmed.ReservationId,
                listing_id = confirmed.ListingId,
                guest_id = confirmed.GuestId,
                check_in = confirmed.Stay.CheckIn,
                check_out = confirmed.Stay.CheckOut,
                guest_count = confirmed.GuestCount,
                confirmed_at = confirmed.ConfirmedAt,
                max_guests = confirmed.MaxGuestsAtConfirmation,
                min_nights = confirmed.MinNightsAtConfirmation,
                max_nights = confirmed.MaxNightsAtConfirmation,
                cancellationToken,
            },
            transaction);

        for (var night = confirmed.Stay.CheckIn; night < confirmed.Stay.CheckOut; night = night.AddDays(1))
        {
            const string unavailableNightSql = """
                INSERT INTO listing_unavailable_nights (
                    listing_id,
                    night,
                    unavailability_type,
                    reservation_id
                )
                VALUES (@listing_id, @night, 'reservation', @reservation_id)
                """;

            try
            {
                await connection.ExecuteAsync(
                    unavailableNightSql,
                    new
                    {
                        listing_id = confirmed.ListingId,
                        night,
                        reservation_id = confirmed.ReservationId,
                        cancellationToken,
                    },
                    transaction);
            }
            catch (PostgresException exception) when (exception.SqlState == PostgresErrorCodes.UniqueViolation)
            {
                throw new ListingUnavailableException();
            }
        }
    }

    private static async Task<GuestBookingEligibility?> LoadGuestEligibility(
        NpgsqlConnection connection,
        NpgsqlTransaction transaction,
        BookStayRequest request,
        CancellationToken cancellationToken)
    {
        const string sql = """
            SELECT booking_eligibility AS BookingEligibility
            FROM guests
            WHERE id = @guest_id
            FOR SHARE
            """;

        var row = await connection.QuerySingleOrDefaultAsync<GuestRow>(
            sql,
            new { guest_id = request.GuestId, cancellationToken },
            transaction);
        var value = row?.BookingEligibility;

        return value switch
        {
            null => null,
            "eligible" => GuestBookingEligibility.Eligible,
            "blocked" => GuestBookingEligibility.Blocked,
            _ => throw new InvalidStoredValueException($"unknown guest booking eligibility: {value}"),
        };
    }

    private static async Task<ListingBookingSettings?> LoadListingBookingSettings(
        NpgsqlConnection connection,
        NpgsqlTransaction transaction,
        BookStayRequest request,
        CancellationToken cancellationToken)
    {
        const string sql = """
            SELECT
                booking_status AS BookingStatus,
                max_guests AS MaxGuests,
                min_nights AS MinNights,
                max_nights AS MaxNights
            FROM listings
            WHERE id = @listing_id
            FOR SHARE
            """;

        var row = await connection.QuerySingleOrDefaultAsync<ListingRow>(
            sql,
            new { listing_id = request.ListingId, cancellationToken },
            transaction);

        if (row is null)
        {
            return null;
        }

        var bookingStatus = row.BookingStatus switch
        {
            "bookable" => ListingBookingStatus.Bookable,
            "disabled" => ListingBookingStatus.Disabled,
            var value => throw new InvalidStoredValueException($"unknown listing booking status: {value}"),
        };

        return new ListingBookingSettings(
            bookingStatus,
            row.MaxGuests,
            row.MinNights,
            row.MaxNights);
    }

    private static async Task<IReadOnlyList<DateOnly>> LoadUnavailableNights(
        NpgsqlConnection connection,
        NpgsqlTransaction transaction,
        BookStayRequest request,
        CancellationToken cancellationToken)
    {
        const string sql = """
            SELECT night AS Night
            FROM listing_unavailable_nights
            WHERE listing_id = @listing_id
              AND night >= @check_in
              AND night < @check_out
            ORDER BY night
            """;

        var rows = await connection.QueryAsync<UnavailableNightRow>(
            sql,
            new
            {
                listing_id = request.ListingId,
                check_in = request.Stay.CheckIn,
                check_out = request.Stay.CheckOut,
                cancellationToken,
            },
            transaction);

        return rows.Select(row => row.Night).ToArray();
    }
}
