## Shared Infrastructure Without Shared Meaning

Shared infrastructure is allowed when it provides mechanics that are not specific to one Domain Capability.

Examples:

```text
database connection pool
transaction handling
context locking
advancing context versions
clock access
ID generation
database error mapping
serialization helpers
logging and tracing
```

Shared infrastructure must not decide business meaning.

Good:

```rust
lock_contexts(&mut tx, &context_locks).await?;
advance_written_context_versions(&mut tx, &context_locks).await?;
```

These functions operate on context keys and lock modes. They do not know what `guest-booking-eligibility:<guest_id>` or `listing-availability:<listing_id>:<night>` means.

Bad:

```rust
booking_consistency_manager.lock_booking_dependencies(...).await?;
reservation_service.update_calendar(...).await?;
shared_domain_model.confirm_reservation(...);
```

These names suggest that shared code owns domain interpretation. That would move meaning away from the capability.

The RPU must derive the command context itself:

```rust
let context_locks = booking_context_locks(&request);
```

The shared infrastructure may then execute the mechanics:

```rust
lock_contexts(&mut tx, &context_locks).await?;
```

Do not let shared infrastructure discover context keys from table names, entity names, or generic conventions. The context is part of the command’s decision and must stay visible in the capability.

Central code may own how a lock is acquired. It must not own why this lock is needed.

Central code may own how a context version is advanced. It must not decide which context was changed.

Central code may map a PostgreSQL unique violation to a technical category. It must not decide the domain rejection unless that mapping is explicitly owned by the capability.

The rule is:

```text
Share mechanics.
Do not share meaning.
```

This keeps the codebase ready to grow. New capabilities can reuse transaction and locking mechanics without being forced into a central domain model.
