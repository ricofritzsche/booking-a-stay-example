using BookingAStay.Capabilities.BookStay;
using BookingAStay.Providers;
using Npgsql;

namespace BookingAStay.Api.Http;

public sealed record BookStayRequestBody(
    Guid GuestId,
    Guid ListingId,
    DateOnly CheckIn,
    DateOnly CheckOut,
    int GuestCount);

public sealed record BookStayConfirmedBody(Guid ReservationId);

public sealed record ApiErrorBody(string Code, string Message);

public static class BookStayEndpoint
{
    public static async Task<IResult> Handle(
        BookStayRequestBody body,
        NpgsqlDataSource dataSource,
        ProviderBundle providers,
        ILoggerFactory loggerFactory,
        CancellationToken cancellationToken)
    {
        var request = new BookStayRequest(
            body.GuestId,
            body.ListingId,
            new Stay(body.CheckIn, body.CheckOut),
            body.GuestCount);

        try
        {
            var response = await Processor.ProcessBookStay(
                request,
                dataSource,
                providers,
                cancellationToken);

            return response switch
            {
                BookStayResponse.Confirmed confirmed => Results.Json(
                    new BookStayConfirmedBody(confirmed.ReservationId),
                    statusCode: StatusCodes.Status201Created),
                BookStayResponse.Rejected rejected => RejectionResponse(rejected.Reason),
                _ => throw new InvalidOperationException("Unknown book stay response."),
            };
        }
        catch (Exception exception) when (!cancellationToken.IsCancellationRequested)
        {
            loggerFactory.CreateLogger(typeof(BookStayEndpoint).FullName!)
                .LogError(exception, "Book stay request failed with a technical error");

            return Results.Json(
                new ApiErrorBody("internal_error", "an internal error occurred"),
                statusCode: StatusCodes.Status500InternalServerError);
        }
    }

    public static (int Status, string Code, string Message) RejectionHttpMapping(BookingRejection rejection)
    {
        return rejection switch
        {
            BookingRejection.InvalidDateRange => (
                StatusCodes.Status422UnprocessableEntity,
                "invalid_date_range",
                "check-out must be after check-in"),
            BookingRejection.InvalidGuestCount => (
                StatusCodes.Status422UnprocessableEntity,
                "invalid_guest_count",
                "guest count must be greater than zero"),
            BookingRejection.StayStartsInPast => (
                StatusCodes.Status422UnprocessableEntity,
                "stay_starts_in_past",
                "check-in date must not be in the past"),
            BookingRejection.GuestNotFound => (
                StatusCodes.Status404NotFound,
                "guest_not_found",
                "guest not found"),
            BookingRejection.GuestBlocked => (
                StatusCodes.Status409Conflict,
                "guest_blocked",
                "guest is blocked from booking"),
            BookingRejection.ListingNotFound => (
                StatusCodes.Status404NotFound,
                "listing_not_found",
                "listing not found"),
            BookingRejection.ListingDisabled => (
                StatusCodes.Status409Conflict,
                "listing_disabled",
                "listing is disabled for booking"),
            BookingRejection.TooManyGuests => (
                StatusCodes.Status422UnprocessableEntity,
                "too_many_guests",
                "guest count exceeds listing capacity"),
            BookingRejection.StayTooShort => (
                StatusCodes.Status422UnprocessableEntity,
                "stay_too_short",
                "stay is shorter than the listing minimum"),
            BookingRejection.StayTooLong => (
                StatusCodes.Status422UnprocessableEntity,
                "stay_too_long",
                "stay is longer than the listing maximum"),
            BookingRejection.ListingUnavailable => (
                StatusCodes.Status409Conflict,
                "listing_unavailable",
                "listing is unavailable for the requested dates"),
            _ => throw new ArgumentOutOfRangeException(nameof(rejection), rejection, null),
        };
    }

    private static IResult RejectionResponse(BookingRejection rejection)
    {
        var (status, code, message) = RejectionHttpMapping(rejection);
        return Results.Json(new ApiErrorBody(code, message), statusCode: status);
    }
}
