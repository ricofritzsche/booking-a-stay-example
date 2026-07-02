# PRINCIPLES AND PATTERNS

## What We Apply

This example does not start with a pattern catalog. The goal is not to prove that every known software design term can be used somewhere. The goal is to show a small, coherent way to build a ready-to-grow application around meaningful business capabilities.

The central idea is simple: a capability processes a request, reads the part of the Application State needed for the decision, applies the domain rules in a Functional Core, and records the result only when the decision context is still valid. In this example, the Application State is stored in PostgreSQL tables, not in an event store.

This matters because we still want the benefits of event-oriented thinking. A successful booking does not produce a generic database update. It produces a meaningful result, such as a confirmed reservation. The difference is that this result is recorded into relational tables.

The design therefore combines a few ideas:

Domain Capabilities define the business units of growth. Request Processing Units implement the processing of concrete requests. Functional Core and Imperative Shell separate decision logic from external work. Command Context Consistency protects the context used for the decision. PostgreSQL provides the relational Application State and the database mechanisms needed to commit results safely.

These ideas do not all operate on the same level. Domain Capabilities describe the business abilities we grow with. A Request Processing Unit implements one Domain Capability through a clear request/response boundary. Functional Core and Imperative Shell keep the decision separate from external work. Command Context Consistency defines when the result of a command may be committed. PostgreSQL stores the Application State and provides the mechanisms we use to protect that commit.

Together they make one flow visible: a request enters the RPU, the shell loads the context needed for the decision, the Functional Core decides, and the shell records the result only when that context is still valid.

## Domain Capability as the Growth Boundary

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
