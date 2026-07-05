# Booking a Stay — Rust Example

Rust implementation of the stay-booking domain. This is the foundation service
(Axum + sqlx + PostgreSQL); domain capabilities live under `src/capabilities/`.

See the repository root (`../DOMAIN.md`, `../specs/`, `../guidelines/`) for the
architecture this code follows.

## Prerequisites

- Rust (stable) via [rustup](https://rustup.rs) — edition 2024, tested on 1.96.
- A reachable PostgreSQL instance with a database named `booking_a_stay`.

### Database setup

The service does **not** create its database on startup — it expects
`booking_a_stay` to already exist and will exit with
`database "booking_a_stay" does not exist` otherwise.

**Option A — use an existing Postgres instance.** Create the database once
(adjust host/user to match your setup):

```bash
createdb -h localhost -U postgres booking_a_stay
# or, from psql:  CREATE DATABASE booking_a_stay;
```

**Option B — start a throwaway Postgres with Docker** (optional; only if you
don't already have an instance). This creates the database for you via
`POSTGRES_DB`:

```bash
docker run --name booking-pg -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=booking_a_stay -p 5432:5432 -d postgres
```

Either option leaves Postgres reachable on `localhost:5432` with the database
the default configuration expects.

## Migrations

Run SQLx migrations from this directory after creating the database:

```bash
sqlx migrate run
```

By default SQLx reads `DATABASE_URL`; use the local development URL if needed:

```bash
DATABASE_URL="postgres://postgres:postgres@localhost:5432/booking_a_stay" sqlx migrate run
```

## Configuration

Settings come from `config.toml`, overridable by `APP_`-prefixed environment
variables (`__` separates nested keys). Defaults expect a local Postgres on
`localhost:5432` (see [Database setup](#database-setup)):

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://postgres:postgres@localhost:5432/booking_a_stay"
max_connections = 10
run_migrations = true

[telemetry]
log_format = "pretty"
log_level = "info"
```

The checked-in URL is for local development only. Deployment should provide its
database connection string through `APP_DATABASE__URL`.

The application runs pending SQLx migrations on startup by default. Disable
that only for environments that apply migrations separately:

```bash
export APP_DATABASE__RUN_MIGRATIONS=false
```

Override examples:

```bash
export APP_SERVER__PORT=9000
export APP_DATABASE__URL="postgres://user:pass@localhost:5432/booking_a_stay"
```

## Run

```bash
cargo run
```

The server binds the configured address and exposes a health check:

```bash
curl http://localhost:8080/health
# {"status":"ok","database":"up"}
```

API routes live under `/api`. Booking a stay is exposed as creating a
reservation:

```bash
POST /api/reservations
```

## Demo data

Migrations seed deterministic demo data for local API usage.

Useful eligible guest:

```text
Manuel Horse
20000000-0000-0000-0000-000000000001
```

Useful bookable listing:

```text
Seaside Apartment with Morning Balcony
30000000-0000-0000-0000-000000000001
```

Blocked guests and disabled listings are also included so rejection paths can
be tried.

## Example requests

Successful bookings return `201 Created` with a generated reservation id.

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000001",
    "listing_id": "30000000-0000-0000-0000-000000000001",
    "check_in": "2026-08-01",
    "check_out": "2026-08-04",
    "guest_count": 2
  }'
```

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000002",
    "listing_id": "30000000-0000-0000-0000-000000000002",
    "check_in": "2026-08-05",
    "check_out": "2026-08-07",
    "guest_count": 2
  }'
```

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000003",
    "listing_id": "30000000-0000-0000-0000-000000000005",
    "check_in": "2026-08-10",
    "check_out": "2026-08-13",
    "guest_count": 4
  }'
```

Expected error responses:

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000009",
    "listing_id": "30000000-0000-0000-0000-000000000003",
    "check_in": "2026-08-01",
    "check_out": "2026-08-04",
    "guest_count": 2
  }'
# 409 Conflict
# {"code":"guest_blocked","message":"guest is blocked from booking"}
```

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000001",
    "listing_id": "30000000-0000-0000-0000-000000000046",
    "check_in": "2026-08-01",
    "check_out": "2026-08-04",
    "guest_count": 2
  }'
# 409 Conflict
# {"code":"listing_disabled","message":"listing is disabled for booking"}
```

```bash
curl -i -X POST http://localhost:8080/api/reservations \
  -H 'content-type: application/json' \
  -d '{
    "guest_id": "20000000-0000-0000-0000-000000000001",
    "listing_id": "30000000-0000-0000-0000-000000000001",
    "check_in": "2026-08-01",
    "check_out": "2026-08-01",
    "guest_count": 2
  }'
# 422 Unprocessable Entity
# {"code":"invalid_date_range","message":"check-out must be after check-in"}
```

## Logging

Structured logging via `tracing`. Control verbosity with `RUST_LOG` or
`APP_LOG_LEVEL`, and switch to JSON output with `APP_LOG_FORMAT=json` or
`APP_TELEMETRY__LOG_FORMAT=json`:

```bash
RUST_LOG=debug APP_LOG_FORMAT=json cargo run
```

Each HTTP request receives or preserves an `x-request-id` header. The request
span records method, path, status, latency, and request ID.

Handlers can extract `telemetry::RequestId` and attach it to `AppError` values
so error responses include the same request ID as the logs.

## Develop

```bash
cargo build          # compile
cargo test           # run tests
cargo clippy         # lints
cargo fmt            # format
```
