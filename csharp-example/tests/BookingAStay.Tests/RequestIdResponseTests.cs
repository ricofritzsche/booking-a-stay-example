using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;

namespace BookingAStay.Tests;

[Collection(PostgreSqlCollection.Name)]
public sealed class RequestIdResponseTests
{
    private const string RequestIdHeader = "x-request-id";
    private readonly PostgreSqlFixture _fixture;

    public RequestIdResponseTests(PostgreSqlFixture fixture)
    {
        _fixture = fixture;
    }

    [Fact]
    public async Task ResponsePropagatesRequestId()
    {
        await using var factory = CreateFactory();
        using var client = factory.CreateClient();
        using var request = new HttpRequestMessage(HttpMethod.Get, "/health");
        request.Headers.Add(RequestIdHeader, "test-request-id");

        using var response = await client.SendAsync(request);

        Assert.Equal("test-request-id", response.Headers.GetValues(RequestIdHeader).Single());
    }

    [Fact]
    public async Task ResponseGeneratesRequestIdWhenMissing()
    {
        await using var factory = CreateFactory();
        using var client = factory.CreateClient();

        using var response = await client.GetAsync("/health");

        Assert.False(string.IsNullOrWhiteSpace(response.Headers.GetValues(RequestIdHeader).Single()));
    }

    private WebApplicationFactory<global::Program> CreateFactory()
    {
        return new WebApplicationFactory<global::Program>()
            .WithWebHostBuilder(builder =>
            {
                builder.UseSetting("ConnectionStrings:ApplicationState", _fixture.ConnectionString);
            });
    }
}
