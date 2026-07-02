## Command Context Consistency

**Command Context Consistency means that the result of a command is committed only when the context the command relied on for its decision is still valid at commit time.**

For this codebase, CCC applies to every command that makes a domain decision and changes the Application State.

The command context must be derived from the command’s intent. It must not be inferred from table names, repository methods, or generic entity boundaries.

For `BookStay`, the command context includes:

```text
guest booking eligibility
listing booking settings
listing availability for the requested nights
```

It does not include unrelated data such as:

```text
guest display name
guest profile photo
listing title
listing description
listing photos
```

A command must make its context explicit before it commits a result. The RPU is responsible for protecting this context while the Functional Core makes the decision and while the shell records the result.

Use the smallest context that is correct for the command. Do not protect the whole user, listing, reservation, or database when only one business-relevant part is needed.

The consistency mechanism depends on the shape of the context:

```text
one existing row -> row lock or row version
uniqueness rule -> unique constraint
range conflict -> exclusion constraint or explicit context
command-defined business context -> consistency context row
```

Do not add a generic version column everywhere and treat that as CCC. A version column can be correct for a row-based context, but CCC protects the command’s decision context, not automatically the physical row that happens to be updated.

If a command changes a context that another command may rely on, it must advance the corresponding context version or use the database constraint that protects the same rule.
