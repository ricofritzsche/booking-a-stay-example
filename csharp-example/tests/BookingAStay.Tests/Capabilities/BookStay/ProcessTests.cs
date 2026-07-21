using BookingAStay.ApplicationState.Db;
using BookingAStay.Capabilities.BookStay;
using BookingAStay.Providers;
using Npgsql;

namespace BookingAStay.Tests.Capabilities.BookStay;

[Collection(PostgreSqlCollection.Name)]
public sealed class ProcessTests
{
    private readonly PostgreSqlFixture _fixture;

    public ProcessTests(PostgreSqlFixture fixture)
    {
        _fixture = fixture;
    }

    [Fact]
    public async Task ConfirmsReservationAndRecordsListingUnavailableNights()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var response = await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        var confirmed = Assert.IsType<BookStayResponse.Confirmed>(response);

        Assert.Equal(1, await Count(dataSource, "reservations"));
        Assert.Equal(confirmed.ReservationId, await StoredReservationId(dataSource));
        Assert.Equal(
            [Date(2026, 8, 1), Date(2026, 8, 2), Date(2026, 8, 3)],
            await UnavailableNights(dataSource, fixture.ListingId));
    }

    [Fact]
    public async Task RecordsBookingTermsUsedAtConfirmation()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedListingWithTerms(dataSource, "eligible", "bookable", 5, 2, 7);

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId) with { GuestCount = 5 };
        var response = await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        Assert.IsType<BookStayResponse.Confirmed>(response);
        Assert.Equal(5, await StoredMaxGuestsAtConfirmation(dataSource));

        await using (var connection = await dataSource.OpenConnectionAsync())
        await using (var command = new NpgsqlCommand("UPDATE listings SET max_guests = 4 WHERE id = @listing_id", connection))
        {
            command.Parameters.AddWithValue("listing_id", fixture.ListingId);
            await command.ExecuteNonQueryAsync();
        }

        Assert.Equal(5, await StoredMaxGuestsAtConfirmation(dataSource));
    }

    [Fact]
    public async Task DoesNotOccupyTheCheckOutDate()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId);
        await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            "SELECT COUNT(*) FROM listing_unavailable_nights WHERE night = @night",
            connection);
        command.Parameters.AddWithValue("night", Date(2026, 8, 4));

        Assert.Equal(0, (long)(await command.ExecuteScalarAsync() ?? 0L));
    }

    [Fact]
    public async Task RejectsWhenGuestIsBlocked()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "blocked", "bookable");

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var response = await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        Assert.Equal(new BookStayResponse.Rejected(BookingRejection.GuestBlocked), response);
        await AssertNoReservationOrUnavailableNights(dataSource);
    }

    [Fact]
    public async Task RejectsWhenListingIsDisabled()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "disabled");

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var response = await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        Assert.Equal(new BookStayResponse.Rejected(BookingRejection.ListingDisabled), response);
        await AssertNoReservationOrUnavailableNights(dataSource);
    }

    [Fact]
    public async Task RejectsWhenRequestedNightIsAlreadyUnavailable()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");

        await using (var connection = await dataSource.OpenConnectionAsync())
        await using (var command = new NpgsqlCommand(
            """
            INSERT INTO listing_unavailable_nights (
                listing_id,
                night,
                unavailability_type,
                reason
            )
            VALUES (@listing_id, @night, 'host_block', 'maintenance')
            """,
            connection))
        {
            command.Parameters.AddWithValue("listing_id", fixture.ListingId);
            command.Parameters.AddWithValue("night", Date(2026, 8, 2));
            await command.ExecuteNonQueryAsync();
        }

        var request = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var response = await Processor.ProcessBookStay(request, dataSource, new ProviderBundle());

        Assert.Equal(new BookStayResponse.Rejected(BookingRejection.ListingUnavailable), response);
        Assert.Equal(0, await Count(dataSource, "reservations"));
    }

    [Fact]
    public async Task TwoOverlappingBookingRequestsCannotBothConfirm()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");

        var firstRequest = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var secondRequest = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var providers = new ProviderBundle();

        var responses = await Task.WhenAll(
            Processor.ProcessBookStay(firstRequest, dataSource, providers),
            Processor.ProcessBookStay(secondRequest, dataSource, providers));

        var confirmedCount = responses.Count(response => response is BookStayResponse.Confirmed);
        var unavailableRejectionCount = responses.Count(response =>
            response is BookStayResponse.Rejected { Reason: BookingRejection.ListingUnavailable });

        Assert.Equal(1, confirmedCount);
        Assert.Equal(1, unavailableRejectionCount);
        Assert.Equal(3, await Count(dataSource, "listing_unavailable_nights"));
        Assert.Equal(3, await DistinctUnavailableNightCount(dataSource));
    }

    [Fact]
    public async Task TwoGuestsWithDisjointStaysBothConfirmConcurrently()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");
        var secondGuestId = await SeedSecondGuest(dataSource);
        var providers = new ProviderBundle();

        var firstRequest = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var secondRequest = BookStayRequest(secondGuestId, fixture.ListingId) with
        {
            Stay = new Stay(Date(2026, 8, 10), Date(2026, 8, 13)),
        };

        var responses = await Task.WhenAll(
            Processor.ProcessBookStay(firstRequest, dataSource, providers),
            Processor.ProcessBookStay(secondRequest, dataSource, providers));

        Assert.IsType<BookStayResponse.Confirmed>(responses[0]);
        Assert.IsType<BookStayResponse.Confirmed>(responses[1]);
        Assert.Equal(2, await Count(dataSource, "reservations"));
        Assert.Equal(6, await Count(dataSource, "listing_unavailable_nights"));
        Assert.Equal(
            [
                Date(2026, 8, 1),
                Date(2026, 8, 2),
                Date(2026, 8, 3),
                Date(2026, 8, 10),
                Date(2026, 8, 11),
                Date(2026, 8, 12),
            ],
            await UnavailableNights(dataSource, fixture.ListingId));
    }

    [Fact]
    public async Task BackToBackStaysBothConfirmConcurrently()
    {
        await using var dataSource = await PrepareDatabase();
        var fixture = await SeedBookableListing(dataSource, "eligible", "bookable");
        var secondGuestId = await SeedSecondGuest(dataSource);
        var providers = new ProviderBundle();

        var firstRequest = BookStayRequest(fixture.GuestId, fixture.ListingId);
        var secondRequest = BookStayRequest(secondGuestId, fixture.ListingId) with
        {
            Stay = new Stay(Date(2026, 8, 4), Date(2026, 8, 7)),
        };

        var responses = await Task.WhenAll(
            Processor.ProcessBookStay(firstRequest, dataSource, providers),
            Processor.ProcessBookStay(secondRequest, dataSource, providers));

        Assert.IsType<BookStayResponse.Confirmed>(responses[0]);
        Assert.IsType<BookStayResponse.Confirmed>(responses[1]);
        Assert.Equal(6, await Count(dataSource, "listing_unavailable_nights"));
        Assert.Equal(
            [
                Date(2026, 8, 1),
                Date(2026, 8, 2),
                Date(2026, 8, 3),
                Date(2026, 8, 4),
                Date(2026, 8, 5),
                Date(2026, 8, 6),
            ],
            await UnavailableNights(dataSource, fixture.ListingId));
    }

    private async Task<NpgsqlDataSource> PrepareDatabase()
    {
        var dataSource = NpgsqlDataSource.Create(_fixture.ConnectionString);

        await using (var connection = await dataSource.OpenConnectionAsync())
        await using (var command = new NpgsqlCommand(
            """
            DROP SCHEMA public CASCADE;
            CREATE SCHEMA public;
            GRANT ALL ON SCHEMA public TO PUBLIC;
            """,
            connection))
        {
            await command.ExecuteNonQueryAsync();
        }

        Migrations.Apply(_fixture.ConnectionString);
        return dataSource;
    }

    private static async Task<BookingFixture> SeedBookableListing(
        NpgsqlDataSource dataSource,
        string guestEligibility,
        string listingStatus)
    {
        return await SeedListingWithTerms(dataSource, guestEligibility, listingStatus, 4, 2, 7);
    }

    private static async Task<BookingFixture> SeedListingWithTerms(
        NpgsqlDataSource dataSource,
        string guestEligibility,
        string listingStatus,
        int maxGuests,
        int minNights,
        int? maxNights)
    {
        var fixture = new BookingFixture(GuidFromByte(1), GuidFromByte(2));

        await using var connection = await dataSource.OpenConnectionAsync();

        await using (var guestCommand = new NpgsqlCommand(
            """
            INSERT INTO guests (id, email, full_name, booking_eligibility)
            VALUES (@id, 'guest@example.test', 'Test Guest', @booking_eligibility)
            """,
            connection))
        {
            guestCommand.Parameters.AddWithValue("id", fixture.GuestId);
            guestCommand.Parameters.AddWithValue("booking_eligibility", guestEligibility);
            await guestCommand.ExecuteNonQueryAsync();
        }

        await using (var listingCommand = new NpgsqlCommand(
            """
            INSERT INTO listings (
                id,
                host_id,
                title,
                max_guests,
                min_nights,
                max_nights,
                booking_status
            )
            VALUES (@id, @host_id, 'Test Listing', @max_guests, @min_nights, @max_nights, @booking_status)
            """,
            connection))
        {
            listingCommand.Parameters.AddWithValue("id", fixture.ListingId);
            listingCommand.Parameters.AddWithValue("host_id", GuidFromByte(4));
            listingCommand.Parameters.AddWithValue("max_guests", maxGuests);
            listingCommand.Parameters.AddWithValue("min_nights", minNights);
            listingCommand.Parameters.AddWithValue("max_nights", (object?)maxNights ?? DBNull.Value);
            listingCommand.Parameters.AddWithValue("booking_status", listingStatus);
            await listingCommand.ExecuteNonQueryAsync();
        }

        return fixture;
    }

    private static async Task<Guid> SeedSecondGuest(NpgsqlDataSource dataSource)
    {
        var guestId = GuidFromByte(3);

        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            """
            INSERT INTO guests (id, email, full_name, booking_eligibility)
            VALUES (@id, 'second-guest@example.test', 'Second Test Guest', 'eligible')
            """,
            connection);
        command.Parameters.AddWithValue("id", guestId);
        await command.ExecuteNonQueryAsync();

        return guestId;
    }

    private static BookStayRequest BookStayRequest(Guid guestId, Guid listingId)
    {
        return new BookStayRequest(
            guestId,
            listingId,
            new Stay(Date(2026, 8, 1), Date(2026, 8, 4)),
            2);
    }

    private static async Task<Guid> StoredReservationId(NpgsqlDataSource dataSource)
    {
        return await Scalar<Guid>(dataSource, "SELECT id FROM reservations");
    }

    private static async Task<int> StoredMaxGuestsAtConfirmation(NpgsqlDataSource dataSource)
    {
        return await Scalar<int>(dataSource, "SELECT max_guests_at_confirmation FROM reservations");
    }

    private static async Task<IReadOnlyList<DateOnly>> UnavailableNights(NpgsqlDataSource dataSource, Guid listingId)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            """
            SELECT night
            FROM listing_unavailable_nights
            WHERE listing_id = @listing_id
            ORDER BY night
            """,
            connection);
        command.Parameters.AddWithValue("listing_id", listingId);
        await using var reader = await command.ExecuteReaderAsync();

        var nights = new List<DateOnly>();
        while (await reader.ReadAsync())
        {
            nights.Add(reader.GetFieldValue<DateOnly>(0));
        }

        return nights;
    }

    private static async Task AssertNoReservationOrUnavailableNights(NpgsqlDataSource dataSource)
    {
        Assert.Equal(0, await Count(dataSource, "reservations"));
        Assert.Equal(0, await Count(dataSource, "listing_unavailable_nights"));
    }

    private static async Task<long> Count(NpgsqlDataSource dataSource, string tableName)
    {
        return await Scalar<long>(dataSource, $"SELECT COUNT(*) FROM {tableName}");
    }

    private static async Task<long> DistinctUnavailableNightCount(NpgsqlDataSource dataSource)
    {
        return await Scalar<long>(
            dataSource,
            "SELECT COUNT(DISTINCT (listing_id, night)) FROM listing_unavailable_nights");
    }

    private static async Task<T> Scalar<T>(NpgsqlDataSource dataSource, string sql)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(sql, connection);
        return (T)(await command.ExecuteScalarAsync() ?? throw new InvalidOperationException("Expected scalar value."));
    }

    private static DateOnly Date(int year, int month, int day)
    {
        return new DateOnly(year, month, day);
    }

    private static Guid GuidFromByte(uint value)
    {
        Span<byte> bytes = stackalloc byte[16];
        bytes[15] = (byte)value;
        return new Guid(bytes, bigEndian: true);
    }

    private sealed record BookingFixture(Guid GuestId, Guid ListingId);
}
