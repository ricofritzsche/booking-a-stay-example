## Database Constraints Where They Fit

Use PostgreSQL constraints when the rule can be expressed directly by the database.

Do not replace good database constraints with custom context locking. A constraint is often the clearest commit guard because it protects the rule at the place where the state is recorded.

Use constraints for rules such as:

```text
required values
valid enum-like states
valid date ranges
foreign key references
unique values
non-overlapping ranges
```

Examples:

```sql
CHECK (check_in < check_out)

CHECK (guest_count > 0)

CHECK (status IN ('confirmed', 'cancelled'))

FOREIGN KEY (guest_id) REFERENCES guests(id)
```

For uniqueness, use a unique constraint or unique index.

Example:

```sql
CREATE UNIQUE INDEX guests_email_unique
ON guests ((lower(email)));
```

Do not protect email uniqueness with a generic command context row. The database can express the rule precisely.

For reservation overlap, PostgreSQL can also protect the rule with an exclusion constraint:

```sql
CREATE EXTENSION IF NOT EXISTS btree_gist;

ALTER TABLE reservations
ADD CONSTRAINT reservations_no_overlap
EXCLUDE USING gist (
    listing_id WITH =,
    daterange(check_in, check_out, '[)') WITH &&
)
WHERE (status = 'confirmed');
```

This means two confirmed reservations for the same listing cannot occupy overlapping nights.

Use this where it fits. It is a strong database-level guard for the reservation calendar.

The Functional Core may still check the same rule before returning a domain result. That gives the caller a normal domain rejection such as:

```rust
BookingRejected::ListingAlreadyReserved
```

But the database constraint remains the final protection at commit time. If the constraint rejects the write, the shell must translate the database error into the correct domain response or process error.

Do not expose constraint names to callers.

Good:

```text
ListingAlreadyReserved
EmailAlreadyUsed
InvalidDateRange
```

Bad:

```text
violates unique index guests_email_unique
violates exclusion constraint reservations_no_overlap
```

Use command context rows when the consistency boundary is command-defined and not naturally protected by one database constraint.

Use database constraints when the rule is a stable invariant of the stored Application State.

The two mechanisms are not competitors. They solve different parts of the same problem. Constraints protect rules the database can express exactly. Command contexts protect the decision context a capability relied on while processing a command.
