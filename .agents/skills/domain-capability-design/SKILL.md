---
name: domain-capability-design
description: Design, implement, and review business logic as explicit Domain Capabilities with Request Processing Units (RPUs) and Command Context Consistency. Use this skill whenever the user designs a new feature or business operation, implements a command or use case, reviews business-logic code, asks about concurrency or consistency in business operations, discusses commands, capabilities, RPUs, decide functions, or Command Context Consistency, or weighs Event Sourcing against relational storage — even if they don't name the pattern explicitly. Also use it when the user asks why an application accepted or rejected a request, or how to make decisions auditable or explainable.
---

# Domain Capability Design

Build business logic so that every decision stays explainable: the path from the command that expresses an intention to the outcome recorded in Application State is explicit in the code, and a developer can follow one business decision without reconstructing the entire application.

## The Principle of One Path

Every business operation follows one path:

```
Command  ──▶  Domain Capability
                │
                ├── obtain the command context (only decision-relevant facts)
                ├── invoke the decide function (pure)
                ├── protect the context during the commit
                ├── record the accepted outcome  /  return the rejection
                └── coordinate required effects (after commit)
```

Definitions that must stay precise:

- **Command**: what someone asked the application to decide. One concrete intention toward a capability. Carries the requester and the request data, nothing more.
- **Domain Capability**: a definition, not code. It names one business responsibility, states the rules behind it, and enumerates every outcome it can produce. It never makes a decision and never names a single command's result — the decision is made per command, by applying the rules to the context. A capability is also not the delivery mechanism; HTTP, queue, or CLI only carry the intention. It is stable, and every request toward it reuses the same definition.
- **Command context**: exactly the information capable of changing the outcome — no generic object graph, no "load everything" entity. Every field must be justified by a rule that reads it.
- **Decide function**: a pure function `decide(command, context) -> Accepted | Rejected(reason)`. The decision is its *result*, not a module. A proposed acceptance becomes final only when the context is confirmed still valid and the result is committed.
- **RPU (Request Processing Unit)**: the executable unit that honors a capability for one concrete command. It loads the context, runs the pure decide function, guards the commit against stale context, records the accepted outcome or returns the rejection, and coordinates effects afterward — behind one visible entry point (e.g. `process_cancel_reservation`). It executes once per command and produces exactly one outcome. Functional Core / Imperative Shell is the recommended construction: the Shell loads context, guards the commit, and handles effects; the Core is the decide function.
- **Command Context Consistency (CCC)**: a command is accepted only when the context it relied on for its decision is still valid at the moment the result is committed. CCC forbids committing on stale context; it does not prescribe the response — rejecting, retrying, or re-evaluating is a per-capability decision.
- **Outcome**: a domain-named result (`ReservationCancelled`, `CancellationRejected: cancellation_period_expired`), never a generic one (`ReservationUpdated`, `outcome: "accepted"`). A rejection is a valid business outcome, not an error.

Capability and RPU are the pair most often collapsed into one. The sharpest way to hold them apart: the capability says what may happen and under which rules; the RPU is where one specific request actually happens and produces exactly one outcome. You can fully describe a capability with no code — responsibility, rules, and outcomes on paper. You cannot run one without an RPU.

Storage is an explicit, separate decision. Relational state (transactions, locks, constraints), an event history (conditional append), or a combination all fulfill the same obligation: the outcome must not be recorded after the basis for the decision has become invalid. Thinking in events is not Event Sourcing; never assume an event store is required.

## Naming Conventions

Derive all names from the capability, consistently:

| Element | Pattern | Example |
| --- | --- | --- |
| Domain Capability | verb phrase | Cancel a Reservation |
| Command | imperative, PascalCase | CancelReservation |
| Context | \<Capability\>Context | CancellationContext |
| Decide function | `decide`  | decide |
| Context loader | `load_context`  | load_context |
| RPU entry point | process_\<command\> | process |
| Accepted outcome | past-tense domain event | ReservationCancelled |
| Rejected outcome | \<Capability\>Rejected + snake_case reason | CancellationRejected: cancellation_period_expired |

The RPU entry point carries the full capability name because it is the visible boundary. Inside the RPU there is only one purpose, so internal functions use short names (`load_context`, `decide`, `commit`) — repeating the capability name inside its own RPU is noise, not clarity.

Timestamps in contexts come from a trusted server-side clock (`evaluatedAt`), never from the caller. State which moment a deadline rule uses (evaluation time vs commit time) and protect accordingly.

## Design a Capability

When the user describes a new feature or business operation, produce a capability specification before any code:

1. **Capability**: name, business responsibility, one sentence.
2. **Command**: fields with types. Only what the caller can legitimately supply.
3. **Outcomes**: every accepted and rejected outcome with its reason codes. Rejections are enumerated up front.
4. **Context**: every field paired with the rule that reads it. A field without a rule is removed. Do not duplicate command fields in the context (the decide function receives both; compare `command.requestedBy` with `context.ownerId` rather than copying).
5. **Rules**: each rule as one testable statement.
6. **Consistency protection**: for each mutable fact in the context, pick a mechanism using the decision table below.
7. **Retention**: what evidence must survive for later explanation (outcome, policy version, context snapshot) given business, legal, and operational requirements. Accepted outcomes are recorded; rejected outcomes are returned, and additionally retained only where audit requirements demand it.

## Consistency Protection

Pick a guard per mutable fact, not per capability — one capability commonly uses several.

| Mechanism | Use when | Guard |
| --- | --- | --- |
| Database constraint | the rule is a stable invariant of stored state that the database expresses exactly | `EXCLUDE`, unique index, `CHECK`. Rejects at write time; the shell maps the error to a domain rejection and never leaks the constraint name |
| Write context row | relational state, and the command changes the fact | lock `FOR UPDATE` before loading state, advance the version on commit |
| Read context row | relational state, and the decision reads the fact without changing it | lock `FOR SHARE`, never advance |
| Conditional append | the outcome is recorded as an event history | `append_if(events, context_query, expected_context_version)`; a `conditional_append_conflict` means the context moved |

Prefer a constraint wherever one expresses the rule exactly — it guards at the place the state is recorded. Use context rows where the boundary is command-defined and no single constraint covers it. They are not competitors.

Conditional append is the optimistic form of the same rule: context rows lock *before* the decision, conditional append detects the collision *at* the commit and rejects the whole batch. The context is an event query rather than a set of keys, and its version is the highest sequence number matching that query. The obligation is identical; only the moment of detection differs. This follows from the storage decision, not from the capability. (Event-store form specified at `architecture.ricofritzsche.me/specifications/command-context-consistency/`.)

Name context keys and query filters from the decision, never from storage:

```text
guest-booking-eligibility:<guest_id>        not  user:<guest_id>
listing-availability:<listing_id>:<night>   not  listing:<listing_id>
```

Renaming a guest must not conflict with `BookStay`; blocking that guest from booking must. The same rule governs event queries — filter on the fact the decision reads, not on every event carrying the entity id. Lock keys in a stable sorted order.