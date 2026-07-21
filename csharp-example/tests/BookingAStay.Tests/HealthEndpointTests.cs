using System.Net;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;

namespace BookingAStay.Tests;

[Collection(PostgreSqlCollection.Name)]
public sealed class HealthEndpointTests
{
    private readonly PostgreSqlFixture _fixture;

    public HealthEndpointTests(PostgreSqlFixture fixture)
    {
        _fixture = fixture;
    }

    [Fact]
    public async Task GetHealthReturnsOk()
    {
        await using var factory = new WebApplicationFactory<global::Program>()
            .WithWebHostBuilder(builder =>
            {
                builder.UseSetting("ConnectionStrings:ApplicationState", _fixture.ConnectionString);
            });
        using var client = factory.CreateClient();

        using var response = await client.GetAsync("/health");
        var body = await response.Content.ReadAsStringAsync();

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        Assert.Equal("{\"status\":\"ok\",\"database\":\"up\"}", body);
    }

    [Fact]
    public async Task HealthReturnsServiceUnavailableWhenDatabaseIsUnreachable()
    {
        const string unreachableConnectionString =
            "Host=127.0.0.1;Port=1;Database=booking_a_stay;Username=postgres;Password=postgres;Timeout=1;Command Timeout=1";
        await using var factory = new WebApplicationFactory<global::Program>()
            .WithWebHostBuilder(builder =>
            {
                builder.UseEnvironment("Testing");
                builder.UseSetting("Testing:SkipMigrations", "true");
                builder.UseSetting("ConnectionStrings:ApplicationState", unreachableConnectionString);
            });
        using var client = factory.CreateClient();

        using var response = await client.GetAsync("/health");
        var body = await response.Content.ReadAsStringAsync();

        Assert.Equal(HttpStatusCode.ServiceUnavailable, response.StatusCode);
        Assert.Equal("{\"status\":\"degraded\",\"database\":\"down\"}", body);
    }
}
