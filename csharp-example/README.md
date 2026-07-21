# Booking A Stay C# Example

Minimal ASP.NET Core implementation skeleton for the stay-booking app. The executable project owns the API, capabilities, application-state mechanics, and providers as folders; there are no layered class-library projects.

Requires the .NET 10 SDK.

Database migrations run through DbUp at application startup. A local development database created before this DbUp migration must be dropped and recreated once because the old `schema_migrations` journal does not carry over to DbUp's `schemaversions` journal.

## Naming

The command record is `BookStayRequest` (not `BookStay` as in the Rust example) to avoid a C# type-versus-namespace collision with `Capabilities.BookStay`.

## CLI

```bash
dotnet build
dotnet run --project src/BookingAStay
dotnet watch --project src/BookingAStay
```

Health check:

```bash
curl -i http://localhost:5000/health
```

Book valid stays using the seeded example data:

```bash
curl -i -X POST http://localhost:5000/api/reservations \
  -H 'Content-Type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000001",
    "listing_id": "30000000-0000-0000-0000-000000000001",
    "check_in": "2027-08-01",
    "check_out": "2027-08-04",
    "guest_count": 2
  }'

curl -i -X POST http://localhost:5000/api/reservations \
  -H 'Content-Type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000002",
    "listing_id": "30000000-0000-0000-0000-000000000002",
    "check_in": "2027-08-10",
    "check_out": "2027-08-13",
    "guest_count": 2
  }'
```

## Tests

Run the complete suite from `csharp-example/`:

```bash
~/.dotnet/dotnet test
```

Run focused Book a Stay integration tests:

```bash
~/.dotnet/dotnet test --filter FullyQualifiedName~ProcessTests
~/.dotnet/dotnet test --filter FullyQualifiedName~BookStayHttpTests
```
