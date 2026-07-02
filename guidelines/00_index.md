# Guidelines

## Sections

1. [Domain Capability](01_domain_capabilities.md)
2. [Functional Core and Imperative Shell](02_functional_core_imperative_shell.md)
3. [Command Context Consistency](03_command_context_consistency.md)
4. [Application State in PostgreSQL](04_application_state_in_postgresql.md)
5. [Event-Oriented Results Without an Event Store](05_event_oriented_results_without_an_event_store.md)
6. [Command Contexts and Commit Guards](06_command_contexts_and_commit_guards.md)
7. [Database Constraints Where They Fit](07_database_constraints_where_they_fit.md)
8. [Command-Owned SQL](08_command_owned_sql.md)
9. [Shared Infrastructure Without Shared Meaning](09_shared_infrastructure_without_shared_meaning.md)
10. [Testing the Functional Core and the RPU](10_testing_the_rpu.md)
11. [Adding New Capabilities](11_adding_new_capabilities.md)
12. [What This Codebase Avoids](12_what_this_codebase_avoids.md)

## What We Apply

This example does not start with a pattern catalog. The goal is not to prove that every known software design term can be used somewhere. The goal is to show a small, coherent way to build a ready-to-grow application around meaningful business capabilities.

The central idea is simple: a capability processes a request, reads the part of the Application State needed for the decision, applies the domain rules in a Functional Core, and records the result only when the decision context is still valid. In this example, the Application State is stored in PostgreSQL tables, not in an event store.

This matters because we still want the benefits of event-oriented thinking. A successful booking does not produce a generic database update. It produces a meaningful result, such as a confirmed reservation. The difference is that this result is recorded into relational tables.

The design therefore combines a few ideas:

Domain Capabilities define the business units of growth. Request Processing Units implement the processing of concrete requests. Functional Core and Imperative Shell separate decision logic from external work. Command Context Consistency protects the context used for the decision. PostgreSQL provides the relational Application State and the database mechanisms needed to commit results safely.

These ideas do not all operate on the same level. Domain Capabilities describe the business abilities we grow with. A Request Processing Unit implements one Domain Capability through a clear request/response boundary. Functional Core and Imperative Shell keep the decision separate from external work. Command Context Consistency defines when the result of a command may be committed. PostgreSQL stores the Application State and provides the mechanisms we use to protect that commit.

Together they make one flow visible: a request enters the RPU, the shell loads the context needed for the decision, the Functional Core decides, and the shell records the result only when that context is still valid.
