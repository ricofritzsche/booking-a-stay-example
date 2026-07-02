## Adding New Capabilities

Add new business behavior as a new **Domain Capability**, not as a change to a central domain model.

Start with the business intent:

```text
Cancel a Reservation
Change a Reservation
Manage Listing Availability
Manage Booking Eligibility
```

Then define the capability boundary:

```text
request
response
loaded state
decision context
domain result
domain rejection
command context
SQL required by this command
```

A new capability may use existing tables. That is expected. Do not create a new table only to avoid sharing Application State. Sharing state is fine. Sharing hidden meaning is not.

For example, `CancelReservation` will use `reservations` and listing availability. It should still own its own request type, context shape, decision function, and SQL functions.

Required structure:

```text
src/
  features/
    cancel_reservation/
      mod.rs
      request.rs
      context.rs
      decide.rs
      process.rs
      sql.rs
```

The capability must define its command context explicitly.

For `CancelReservation`, this may include:

```text
reservation-status:<reservation_id>
listing-availability:<listing_id>:<night>
```

For `ChangeReservation`, this may include:

```text
reservation-status:<reservation_id>
listing-availability:<listing_id>:<old-night>
listing-availability:<listing_id>:<new-night>
```

For `ManageListingAvailability`, this may include:

```text
listing-availability:<listing_id>:<night>
```

For `ManageBookingEligibility`, this may include:

```text
guest-booking-eligibility:<guest_id>
listing-booking-settings:<listing_id>
```

Use the same context key when two capabilities affect the same business-relevant context. Otherwise CCC can be bypassed.

Do not reuse a broad key only because it is convenient.

Avoid:

```text
listing:<listing_id>
guest:<guest_id>
reservation:<reservation_id>
```

Prefer keys that name the business context:

```text
listing-availability:<listing_id>:<night>
guest-booking-eligibility:<guest_id>
reservation-status:<reservation_id>
```

When adding a capability, follow this order:

```text
describe the capability from the domain perspective
define request and response
define loaded state and decision context
write the Functional Core
write Functional Core tests
define command context keys
write SQL load and record functions
write the RPU
write RPU integration tests
add concurrency tests when the command changes shared context
```

Do not start with database tables. Start with the capability and the decision it has to make. Add or change tables only when the capability needs Application State that does not exist yet.

Existing capabilities should not be modified just to make the new capability fit a shared model. If two capabilities need the same mechanical helper, extract the helper. If they only share a table, keep their interpretation separate.
