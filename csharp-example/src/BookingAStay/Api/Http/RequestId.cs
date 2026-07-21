namespace BookingAStay.Api.Http;

public sealed class RequestIdMiddleware
{
    public const string HeaderName = "x-request-id";

    private readonly RequestDelegate _next;
    private readonly ILogger<RequestIdMiddleware> _logger;

    public RequestIdMiddleware(RequestDelegate next, ILogger<RequestIdMiddleware> logger)
    {
        _next = next;
        _logger = logger;
    }

    public async Task InvokeAsync(HttpContext context)
    {
        var requestId = context.Request.Headers.TryGetValue(HeaderName, out var suppliedRequestId)
            ? suppliedRequestId.ToString()
            : Guid.NewGuid().ToString();

        context.TraceIdentifier = requestId;
        context.Response.Headers[HeaderName] = requestId;

        using (_logger.BeginScope(new Dictionary<string, object>
        {
            ["request_id"] = requestId,
        }))
        {
            await _next(context);
        }
    }
}

public static class RequestIdExtensions
{
    public static IApplicationBuilder UseRequestId(this IApplicationBuilder app)
    {
        return app.UseMiddleware<RequestIdMiddleware>();
    }
}
