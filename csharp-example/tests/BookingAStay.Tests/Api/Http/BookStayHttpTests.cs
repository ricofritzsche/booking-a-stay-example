using System.Net;
using System.Text;
using System.Text.Json;
using BookingAStay.ApplicationState.Db;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Npgsql;

namespace BookingAStay.Tests.Api.Http;

[Collection(PostgreSqlCollection.Name)]
public sealed class BookStayHttpTests
{
    private readonly PostgreSqlFixture _fixture;

    public BookStayHttpTests(PostgreSqlFixture fixture)
    {
        _fixture = fixture;
    }

    [Fact]
    public async Task ConfirmsReservationThroughHttp()
    {
        await using var dataSource = await PrepareDatabase();
        var booking = await SeedBookableListing(dataSource, "eligible");
        await using var factory = CreateFactory();
        using var client = factory.CreateClient();

        using var response = await PostBooking(client, booking, 2);
        var body = await response.Content.ReadAsStringAsync();
        using var json = JsonDocument.Parse(body);
        var reservationId = json.RootElement.GetProperty("reservation_id").GetGuid();

        Assert.Equal(HttpStatusCode.Created, response.StatusCode);
        Assert.False(json.RootElement.TryGetProperty("reservationId", out _));
        Assert.True(await ReservationExists(dataSource, reservationId, booking));
    }

    [Fact]
    public async Task BlockedGuestReturnsConflict()
    {
        await using var dataSource = await PrepareDatabase();
        var booking = await SeedBookableListing(dataSource, "blocked");
        await using var factory = CreateFactory();
        using var client = factory.CreateClient();

        using var response = await PostBooking(client, booking, 2);
        var body = await response.Content.ReadAsStringAsync();
        using var json = JsonDocument.Parse(body);

        Assert.Equal(HttpStatusCode.Conflict, response.StatusCode);
        Assert.Equal("guest_blocked", json.RootElement.GetProperty("code").GetString());
    }

    [Fact]
    public async Task NegativeGuestCountReturnsUnprocessableEntity()
    {
        await using var dataSource = await PrepareDatabase();
        var booking = await SeedBookableListing(dataSource, "eligible");
        await using var factory = CreateFactory();
        using var client = factory.CreateClient();

        using var response = await PostBooking(client, booking, -1);
        var body = await response.Content.ReadAsStringAsync();
        using var json = JsonDocument.Parse(body);

        Assert.Equal(HttpStatusCode.UnprocessableEntity, response.StatusCode);
        Assert.Equal("invalid_guest_count", json.RootElement.GetProperty("code").GetString());
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
        string guestEligibility)
    {
        var fixture = new BookingFixture(Guid.NewGuid(), Guid.NewGuid());

        await using var connection = await dataSource.OpenConnectionAsync();
        await using (var guestCommand = new NpgsqlCommand(
            """
            INSERT INTO guests (id, email, full_name, booking_eligibility)
            VALUES (@id, @email, 'HTTP Test Guest', @booking_eligibility)
            """,
            connection))
        {
            guestCommand.Parameters.AddWithValue("id", fixture.GuestId);
            guestCommand.Parameters.AddWithValue("email", $"{fixture.GuestId}@example.test");
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
            VALUES (@id, @host_id, 'HTTP Test Listing', 4, 2, 7, 'bookable')
            """,
            connection))
        {
            listingCommand.Parameters.AddWithValue("id", fixture.ListingId);
            listingCommand.Parameters.AddWithValue("host_id", Guid.NewGuid());
            await listingCommand.ExecuteNonQueryAsync();
        }

        return fixture;
    }

    private static async Task<HttpResponseMessage> PostBooking(
        HttpClient client,
        BookingFixture booking,
        int guestCount)
    {
        var json = $$"""
            {
              "guest_id": "{{booking.GuestId}}",
              "listing_id": "{{booking.ListingId}}",
              "check_in": "2030-08-01",
              "check_out": "2030-08-04",
              "guest_count": {{guestCount}}
            }
            """;
        using var content = new StringContent(json, Encoding.UTF8, "application/json");
        return await client.PostAsync("/api/reservations", content);
    }

    private static async Task<bool> ReservationExists(
        NpgsqlDataSource dataSource,
        Guid reservationId,
        BookingFixture booking)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            """
            SELECT EXISTS (
                SELECT 1
                FROM reservations
                WHERE id = @reservation_id
                  AND guest_id = @guest_id
                  AND listing_id = @listing_id
            )
            """,
            connection);
        command.Parameters.AddWithValue("reservation_id", reservationId);
        command.Parameters.AddWithValue("guest_id", booking.GuestId);
        command.Parameters.AddWithValue("listing_id", booking.ListingId);
        return (bool)(await command.ExecuteScalarAsync() ?? false);
    }

    private WebApplicationFactory<global::Program> CreateFactory()
    {
        return new WebApplicationFactory<global::Program>()
            .WithWebHostBuilder(builder =>
            {
                builder.UseSetting("ConnectionStrings:ApplicationState", _fixture.ConnectionString);
            });
    }

    private sealed record BookingFixture(Guid GuestId, Guid ListingId);
}
