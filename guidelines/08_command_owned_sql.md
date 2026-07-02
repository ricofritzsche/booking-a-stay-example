## Command-Owned SQL

SQL required for a command belongs close to the capability that uses it.

For `BookStay`, the SQL should live under the `book_stay` feature, not in a generic `ReservationRepository`, `ListingRepository`, or `GuestRepository`.

Recommended shape:

```text
src/
  features/
    book_stay/
      request.rs
      context.rs
      decide.rs
      process.rs
      sql.rs
```

The SQL module should expose functions that match the command flow:

```rust
load_booking_state(...)
record_reservation_confirmed(...)
```

Avoid generic persistence functions such as:

```rust
load_guest(...)
save_reservation(...)
update_listing(...)
```

These names pull the code back toward table-centered design. The RPU does not load objects to mutate them. It loads the Application State required for the command decision and records the domain result returned by the Functional Core.

For `BookStay`, prefer:

```rust
pub async fn load_booking_state(
    tx: &mut Transaction<'_, Postgres>,
    request: &BookStay,
) -> Result<LoadedBookingState, sqlx::Error>
```

and:

```rust
pub async fn record_reservation_confirmed(
    tx: &mut Transaction<'_, Postgres>,
    confirmed: &ReservationConfirmed,
) -> Result<(), sqlx::Error>
```

The loaded state shape belongs to the capability:

```rust
pub struct LoadedBookingState {
    pub guest_booking_status: Option<String>,
    pub listing_booking_status: Option<String>,
    pub listing_max_guests: Option<i32>,
    pub listing_min_nights: Option<i32>,
    pub listing_max_nights: Option<i32>,
    pub has_overlapping_reservation: bool,
}
```

Do not introduce shared entity types just because several capabilities read from the same tables. If `BookStay` needs guest booking status and `ChangeGuestProfile` needs display name and photo, they should not be forced through the same `Guest` object.

A capability may query several tables. That is expected.

```text
BookStay may read guests, listings, reservations, and availability.
CancelReservation may read reservations and listing availability.
ManageListingAvailability may read listings, reservations, and host blocks.
```

The important rule is not table ownership. The rule is that the SQL must make the command context visible.

Do not open or commit transactions inside command SQL functions. The RPU owns the transaction boundary.

Good:

```rust
load_booking_state(&mut tx, &request).await?;
record_reservation_confirmed(&mut tx, &confirmed).await?;
```

Bad:

```rust
reservation_repository.save(confirmed).await?;
```

where the repository opens its own transaction or hides what is written.

Shared SQL helpers are allowed only for mechanics:

```text
locking consistency contexts
advancing context versions
mapping database errors
common pagination helpers
simple type conversions
```

Shared helpers must not decide which business context a command uses. That remains inside the capability.

Small SQL duplication is acceptable when it keeps the command readable. Do not extract a generic repository just to remove a few repeated lines. Refactor when the extracted code is mechanical, stable, and does not hide the decision context.
