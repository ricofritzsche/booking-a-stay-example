## What This Codebase Avoids

Do not introduce a central domain model.

Avoid structures where every capability has to load and mutate the same shared objects:

```text
Guest
Listing
Reservation
Booking
Calendar
```

Shared tables are fine. Shared mutable domain objects are not the design goal.

Do not use DDD aggregates as the default consistency boundary.

A command context is defined by the decision the command has to make. It is not automatically defined by:

```text
guest id
listing id
reservation id
table name
aggregate root
repository method
```

Do not use generic repositories as the main abstraction.

Avoid:

```rust
guest_repository.load(...)
listing_repository.load(...)
reservation_repository.save(...)
```

when these functions hide which data was needed for the command decision.

Prefer command-owned SQL:

```rust
load_booking_state(...)
record_reservation_confirmed(...)
```

Do not hide database transactions inside lower-level persistence functions.

The RPU owns the request processing flow and the transaction boundary. SQL functions may execute queries inside the transaction they receive, but they must not open, commit, or roll back their own transaction.

Do not put domain decisions into SQL.

SQL loads Application State and records results. The Functional Core decides.

Avoid:

```sql
UPDATE reservations
SET status = 'confirmed'
WHERE ...
```

when the `WHERE` clause silently contains the business decision. The SQL may protect the commit, but the domain decision must remain visible in the Functional Core.

Do not turn current-state tables into fake Event Sourcing.

This app can use event-oriented results such as:

```rust
ReservationConfirmed
ReservationCancelled
ListingNightsBlocked
```

But unless these facts are stored as an ordered history and used as the source of truth, this is not Event Sourcing. Do not claim replay, audit history, projections, or temporal queries unless the implementation actually supports them.

Do not overuse `consistency_contexts`.

Use database constraints when the database can express the rule directly. Use context rows when the command-defined context is not naturally protected by one row or one constraint.

Do not make context keys broader than needed.

Avoid:

```text
listing:<listing_id>
guest:<guest_id>
```

when the actual context is:

```text
listing-availability:<listing_id>:<night>
guest-booking-eligibility:<guest_id>
```

Do not share domain meaning through infrastructure.

Shared infrastructure may lock context rows, advance versions, map database errors, provide clocks, generate IDs, and manage transactions. It must not decide which context a command uses or what a domain result means.

Do not claim that Rust enforces Functional Core and Imperative Shell.

Rust makes the separation visible when the core stays synchronous and the shell performs async external work. The separation still has to be maintained by design.

Do not add abstractions before they are needed.

Small duplication inside capabilities is acceptable when it keeps the command flow clear. Extract shared code only when it is mechanical and does not hide the command context, decision logic, or consistency rule.
