## Command Contexts and Commit Guards

Every command that changes the Application State must define its command context explicitly.

A command context describes the part of the Application State that must remain valid while the command is processed. It is derived from the command’s intent, not from table names.

For `BookStay`, use context keys like:

```text
guest-booking-eligibility:<guest_id>
listing-booking-settings:<listing_id>
listing-availability:<listing_id>:<night>
```

Use two context modes:

```rust
pub enum ContextLockMode {
    Read,
    Write,
}
```

A **read context** is context the command relies on but does not change.

Example:

```text
guest-booking-eligibility:<guest_id>
listing-booking-settings:<listing_id>
```

`BookStay` relies on these contexts, but confirming a reservation does not change guest eligibility or listing settings.

A **write context** is context the command changes when the result is committed.

Example:

```text
listing-availability:<listing_id>:<night>
```

`BookStay` changes availability for each booked night, so these contexts must be locked as write contexts and advanced after the reservation is recorded.

Use a shared table for explicit command contexts:

```sql
CREATE TABLE consistency_contexts (
    context_key text PRIMARY KEY,
    version bigint NOT NULL DEFAULT 0
);
```

The table is shared infrastructure. It does not define domain meaning. The RPU defines the context keys.

Before loading the Application State used for the decision, the RPU must lock the relevant context rows.

Use `FOR SHARE` for read contexts:

```sql
SELECT version
FROM consistency_contexts
WHERE context_key = $1
FOR SHARE;
```

Use `FOR UPDATE` for write contexts:

```sql
SELECT version
FROM consistency_contexts
WHERE context_key = $1
FOR UPDATE;
```

Insert missing context rows before locking them:

```sql
INSERT INTO consistency_contexts (context_key, version)
VALUES ($1, 0)
ON CONFLICT (context_key) DO NOTHING;
```

Lock context keys in a stable order.

```rust
locks.sort_by(|a, b| a.key.cmp(&b.key));
```

After the Functional Core returns an accepted domain result, the shell records the result and advances only the written contexts:

```sql
UPDATE consistency_contexts
SET version = version + 1
WHERE context_key = $1;
```

Do not advance read contexts. A command that only reads guest eligibility must not change the guest eligibility context version.

The required flow is:

```text
derive command context
open transaction
insert missing context rows
lock context rows
load Application State
build decision context
call Functional Core
record domain result
advance written context versions
commit
```

Do not load the decision state before locking the command context. The lock must protect the state that the decision will rely on.

Do not use one broad key when narrower keys are correct.

Avoid:

```text
user:<guest_id>
listing:<listing_id>
reservation:<reservation_id>
```

Prefer keys that name the business-relevant context:

```text
guest-booking-eligibility:<guest_id>
listing-booking-settings:<listing_id>
listing-availability:<listing_id>:<night>
```

This prevents unrelated commands from conflicting. Changing a guest display name should not conflict with `BookStay`. Blocking the guest from booking should conflict with `BookStay`, because it changes the context the booking decision depends on.
