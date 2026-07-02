## Event-Oriented Results Without an Event Store

Use meaningful domain results, even though the Application State is stored in PostgreSQL tables.

A successful command should return a result that describes what happened in the domain:

```rust
ReservationConfirmed
ReservationCancelled
ReservationChanged
ListingNightsBlocked
ListingNightsReleased
GuestBookingBlocked
ListingDisabled
```

A rejected command should return a domain rejection:

```rust
BookingRejected
CancellationRejected
ReservationChangeRejected
AvailabilityChangeRejected
EligibilityChangeRejected
```

Do not let the Functional Core return database operations.

Avoid results like:

```text
InsertReservation
UpdateReservationStatus
DeleteAvailabilityRow
SaveGuest
```

The Functional Core should not know how the result is recorded. It decides whether the command can be accepted and returns the domain result.

For `BookStay`:

```rust
pub fn decide(
    request: &BookStay,
    context: &BookingContext,
    confirmed_at: DateTime<Utc>,
) -> Result<ReservationConfirmed, BookingRejected>
```

The Imperative Shell maps the domain result to PostgreSQL:

```text
ReservationConfirmed -> record confirmed reservation in reservations
BookingRejected -> rollback / return rejection response
```

This keeps the code event-oriented without requiring an event store. The capability still thinks in facts and decisions. The difference is that the resulting fact is recorded into relational Application State instead of being appended to an event log.

Do not claim this is Event Sourcing. It is not. The relational tables store the current Application State. The domain result is a boundary between decision logic and persistence, not a guarantee that every fact is stored historically.

If a capability later needs history, auditability, replay, or projections, that has to be introduced deliberately. Do not hide that requirement behind current-state tables.
