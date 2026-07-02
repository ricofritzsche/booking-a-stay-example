# Domain Capability

A **Domain Capability** is the top-level unit for adding business behavior.

Examples in this app:

```text
Book a Stay
Cancel a Reservation
Change a Reservation
Manage Listing Availability
Manage Booking Eligibility
```

Each capability owns:

```text
request type
response type
loaded state shape
decision context
decision function
domain result
SQL required to load and record state
consistency context used by the command
```

A capability may read from and write to shared Application State. It does not own database tables exclusively. Ownership means: the capability owns the interpretation of the data it needs for its decision.

A **Request Processing Unit** is the concrete implementation of one Domain Capability. It exposes the processing boundary:

```rust
process(request) -> response
```

The RPU coordinates the full request flow:

```text
receive request
load required Application State
build decision context
call Functional Core
record result
commit only when the command context is still valid
```

Do not introduce a central domain model to make capabilities share behavior. Do not route capability behavior through generic repositories. Shared code is allowed only for mechanics, such as transactions, SQL helpers, context locking, clocks, IDs, and error mapping.

When a new business behavior is added, prefer adding a new capability next to the existing ones. Reuse the shared Application State, but keep the decision logic, context shape, and consistency rule inside the capability that executes the command.
