## Testing the Functional Core and the RPU

Test the **Functional Core** without PostgreSQL, SQLx, HTTP, mocks, or async runtime setup.

Functional Core tests should construct the request and the decision context directly:

```rust
let request = BookStay { ... };

let context = BookingContext {
    guest_exists: true,
    guest_can_book: true,
    listing_exists: true,
    listing_can_be_booked: true,
    max_guests: Some(4),
    min_nights: Some(2),
    max_nights: Some(30),
    has_overlapping_reservation: false,
};

let result = decide(&request, &context, confirmed_at);
```

Test the domain outcomes:

```text
ReservationConfirmed
BookingRejected::GuestBlocked
BookingRejected::ListingDisabled
BookingRejected::TooManyGuests
BookingRejected::StayTooShort
BookingRejected::StayTooLong
BookingRejected::ListingAlreadyReserved
```

Do not test the Functional Core by setting up database state. The core receives data. It does not load it.

RPU tests should verify the full request flow:

```text
request
derive command context
lock context
load Application State
build decision context
call Functional Core
record result
advance written context versions
commit or rollback
```

Use integration tests for the RPU because it coordinates PostgreSQL transactions, SQLx queries, constraints, and context locking.

RPU tests should cover:

```text
confirmed reservation is recorded
rejected booking does not record a reservation
written context versions are advanced after success
read context versions are not advanced by BookStay
overlapping bookings cannot both be confirmed
database constraint errors are mapped correctly
```

Do not mock SQLx for RPU tests. If the RPU test needs database behavior, use a real PostgreSQL test database.

Keep test structure close to the feature:

```text
tests/
  features/
    book_stay/
      decide_tests.rs
      process_tests.rs
```

Use Functional Core tests for rules. Use RPU tests for integration behavior.

Do not duplicate every core rule through the RPU. One or two representative rejection paths are enough at the RPU level. The detailed rule matrix belongs to the Functional Core tests.

The goal is:

```text
many fast tests for decisions
fewer integration tests for request processing
specific concurrency tests for CCC behavior
```
