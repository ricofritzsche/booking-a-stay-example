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

[telemetry]
log_format = "pretty"
log_level = "info"
```

The checked-in URL is for local development only. Deployment should provide its
database connection string through `APP_DATABASE__URL`.

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

## Logging

Structured logging via `tracing`. Control verbosity with `RUST_LOG` or
`APP_LOG_LEVEL`, and switch to JSON output with `APP_LOG_FORMAT=json` or
`APP_TELEMETRY__LOG_FORMAT=json`:

```bash
RUST_LOG=debug APP_LOG_FORMAT=json cargo run
```

Each HTTP request receives or preserves an `x-request-id` header. The request
span records method, path, status, latency, and request ID.

## Develop

```bash
cargo build          # compile
cargo test           # run tests
cargo clippy         # lints
cargo fmt            # format
```
