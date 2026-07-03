# AGENTS.md

## Project Orientation

This repository defines a **stay booking domain** using **Autonomous Domain Capabilities**. Each capability is implemented as a self-contained **Request Processing Unit (RPU)**.

The repository root is language-neutral. It contains domain documentation, capability specifications, and implementation guidance. Do **not** place application code in the repository root.

Concrete implementations live in dedicated folders:


```
rust-example/
go-example/
typescript-example/
```


Start with `rust-example/` unless told otherwise.

## Repository Structure

- `rust-example/src/api/` — Thin delivery layer. Translates external requests (HTTP, CLI, etc.) into capability requests and converts capability responses back into external formats. Keep this layer minimal.
- `rust-example/src/capabilities/` — Contains the RPUs. Each subfolder implements one domain capability and owns its full request processing path.
- `rust-example/src/application_state/` — Shared PostgreSQL infrastructure and persistence mechanics only. Contains no domain logic or business rules.
- `rust-example/src/providers/` — External dependencies (clock, ID generation, payment services, etc.). These are thin adapters.
- `rust-example/src/reactors/` — Coordination logic needed when one external interaction requires multiple RPUs or providers. Keep empty until genuinely required.
- `specs/` — Capability specifications (business behavior).
- `migrations/` — Database schema migrations.
- `tests/` — Cross-cutting and integration tests.
- `guidelines/` — Architectural rules and decisions (read these first).

## Capability Structure

Each capability must keep its complete request processing path together in one folder. This is the core rule of the architecture.

A recommended structure for a capability (e.g. `book_a_stay/`) is:

- `core.rs` — Functional Core: pure, synchronous decision logic.
- `shell.rs` — Imperative Shell: transaction management, loading state, calling the core, recording results, and enforcing consistency.
- `context.rs` — Explicit definition of the decision context this capability protects (used for Command Context Consistency).
- `request.rs` / `response.rs` — Input and output types for this capability.
- `sql.rs` — All SQL queries owned by this capability (no generic repositories).
- `mod.rs` — Public module exports.

The exact file split can vary with complexity, but **all decision logic, context definition, SQL, and result recording for this capability must stay inside its folder**. Do not scatter logic across multiple capabilities or into shared layers.

## Implementation Rules

- The **Functional Core** is synchronous and operates only on explicit data. It must not perform I/O, access the database, call providers, or depend on async runtimes.
- The **Imperative Shell** owns all side effects: transactions, SQL, clocks, ID generation, and provider calls.
- SQL belongs inside the capability that owns the command (`sql.rs`). Do not create generic repositories or shared data access layers.
- There is **no central domain model**. Do not create rich entities or aggregates that accumulate behavior across capabilities.
- Shared infrastructure (`application_state/`, `providers/`) owns mechanics only. It must not contain business meaning or decision logic.
- Read the `guidelines/` folder for deeper rationale.

## Command Context Consistency (CCC)

Every command capability must explicitly define and protect the narrow set of facts required for its decision.

Rules:
- The capability owns its context definition (usually in `context.rs`).
- Lock the relevant context **before** loading the decision state.
- After the Functional Core returns a decision, re-validate or use a conditional write (e.g. `INSERT ... ON CONFLICT` or version check) before committing.
- Only advance context versions or release locks for data this capability actually modified.
- Prefer database constraints (unique constraints, exclusion constraints) when they naturally enforce the business rule.

Context is always capability-specific. Never reuse a generic "entity version" for multiple unrelated decisions.

## Testing Instructions

- **Functional Core tests**: Test `core.rs` in isolation with no database, no async, and no external dependencies. These tests should be fast and deterministic.
- **RPU / Shell tests**: Test the full `process` flow (shell + core) using a real PostgreSQL connection (test containers or dedicated test database). Cover happy path, rejection reasons, and error mapping.
- **Concurrency tests**: Explicitly test Command Context Consistency behavior — concurrent bookings, overlapping date ranges, and context staleness scenarios.
- Run tests with:
  ```bash
  cargo test

Use focused test commands once the capability exists (e.g. `cargo test book_a_stay`).

## Coding Style (Rust)

- Prefer explicit, domain-meaningful types over generic DTOs or primitive obsession.
- Keep the Functional Core completely free of `async`, hidden side effects, or framework dependencies.
- Write readable, self-documenting SQL in `sql.rs`. SQL is part of the capability’s logic.
- Do not mock database behavior in tests. Use real PostgreSQL for integration and concurrency tests.
- Return domain-specific errors from the Functional Core. Map technical errors only in the Imperative Shell.

## Documentation Rules

- Keep `DOMAIN.md` and `specs/` business-focused.
- Keep implementation documentation (inside capability folders or `guidelines/`) technical.
- The root `README.md` should stay short and orientation-only.
- Never mix business domain language with technical implementation details in the same document.
- When behavior changes, update the corresponding spec in `specs/` and the relevant guideline if needed.

## Anti-Patterns (What Agents Must Avoid)

- Creating a shared `domain/` or `models/` folder with rich entities or aggregates.
- Introducing generic repositories or shared data access abstractions.
- Scattering SQL or decision logic across multiple files or layers.
- Putting business rules inside `application_state/` or providers.
- Using a single generic version column for all consistency needs.
- Making the Functional Core perform I/O or depend on async.
