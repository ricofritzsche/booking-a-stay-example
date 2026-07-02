## Application State in PostgreSQL

In this app, PostgreSQL stores the **Application State**.

Tables are used to persist the current state required by the capabilities:

```text
guests
listings
reservations
listing availability
consistency contexts
```

The tables do not own the domain behavior. They store state. The capability owns the interpretation of the data it reads and the rules it applies before recording a result.

A capability may read from and write to several tables. A table may be used by several capabilities. Do not treat table ownership as capability ownership.

For example, `BookStay`, `CancelReservation`, and `ChangeReservation` all interact with reservations and listing availability. They do not share one central reservation object. Each capability reads the Application State it needs and interprets it for its own command.

SQL should stay close to the capability that needs it. Avoid generic repositories that hide which data was loaded for the decision. A repository-style abstraction is only acceptable for pure infrastructure mechanics, not for hiding command-specific context.

The Application State can grow in small steps. Add tables, columns, indexes, and constraints when a capability needs them. Do not design a complete relational model upfront for capabilities that do not exist yet.

PostgreSQL mechanisms should be used where they express the rule clearly:

```text
foreign keys for references
check constraints for simple invariants
unique constraints for uniqueness
indexes for query performance
row locks for protected existing rows
consistency context rows for command-defined contexts
```

The schema is shared infrastructure. The meaning of the data stays inside the capability that uses it.
