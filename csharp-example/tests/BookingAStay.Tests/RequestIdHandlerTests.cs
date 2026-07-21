using BookingAStay.Api.Http;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.TestHost;

namespace BookingAStay.Tests;

public sealed class RequestIdHandlerTests
{
    [Fact]
    public async Task RequestIdIsAvailableInsideHandlers()
    {
        var builder = WebApplication.CreateBuilder();
        builder.WebHost.UseTestServer();
        await using var app = builder.Build();
        app.UseRequestId();
        app.UseRouting();
        app.MapGet("/request-id", (HttpContext context) => Results.Text(context.TraceIdentifier));
        await app.StartAsync();

        using var client = app.GetTestClient();
        using var request = new HttpRequestMessage(HttpMethod.Get, "/request-id");
        request.Headers.Add("x-request-id", "handler-request-id");

        using var response = await client.SendAsync(request);
        var body = await response.Content.ReadAsStringAsync();

        Assert.Equal("handler-request-id", body);
    }
}
