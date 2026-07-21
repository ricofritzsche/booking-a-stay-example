namespace BookingAStay.Api.Http;

public static class Routes
{
    public static IEndpointRouteBuilder MapApiRoutes(this IEndpointRouteBuilder app)
    {
        app.MapGet("/health", HealthEndpoint.Handle);
        app.MapGroup("/api")
            .MapPost("/reservations", BookStayEndpoint.Handle);

        return app;
    }
}
