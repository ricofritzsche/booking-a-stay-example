# Functional Core and Imperative Shell

Each RPU must separate decision logic from external work.

The **Functional Core** contains the domain decision. It receives explicit data and returns a domain result or a domain rejection.

For `BookStay`:

```rust
pub fn decide(
    request: &BookStay,
    context: &BookingContext,
    confirmed_at: DateTime<Utc>,
) -> Result<ReservationConfirmed, BookingRejected>
```

The Functional Core must not:

```text
open transactions
run SQL
call providers
read the clock directly
generate IDs directly
perform logging as part of the decision
depend on async runtime behavior
```

The **Imperative Shell** performs the work outside the decision:

```rust
pub async fn process(
    request: BookStay,
    pool: &PgPool,
    clock: &Clock,
) -> Result<BookStayResponse, ProcessBookStayError>
```

The shell is responsible for:

```text
receiving the request
opening the transaction
locking the command context
loading Application State
building the decision context
calling the Functional Core
recording the domain result
advancing changed context versions
committing or rolling back the transaction
mapping technical failures to process errors
```

Rust makes this boundary easy to see. The core is a normal synchronous function over data. The shell is usually async because it talks to PostgreSQL, providers, clocks, or other external systems.

Do not claim that Rust enforces purity. It does not. A normal Rust function can still perform I/O or access shared state. The separation is a design rule for this codebase.

Keep the Functional Core small enough to test without PostgreSQL, SQLx, HTTP, mocks, or async runtime setup. Tests for the core should construct the request and context directly and assert the returned domain result.
