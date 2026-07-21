using Npgsql;

namespace BookingAStay.Api.Http;

public sealed record HealthResponse(string Status, string Database);

public static class HealthEndpoint
{
    public static async Task<IResult> Handle(
        NpgsqlDataSource dataSource,
        CancellationToken cancellationToken)
    {
        try
        {
            await using var connection = await dataSource.OpenConnectionAsync(cancellationToken);
            await using var command = new NpgsqlCommand("SELECT 1", connection);
            await command.ExecuteNonQueryAsync(cancellationToken);

            return Results.Ok(new HealthResponse("ok", "up"));
        }
        catch (Exception) when (!cancellationToken.IsCancellationRequested)
        {
            return Results.Json(
                new HealthResponse("degraded", "down"),
                statusCode: StatusCodes.Status503ServiceUnavailable);
        }
    }
}
