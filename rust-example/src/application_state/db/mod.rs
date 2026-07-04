//! Shared database mechanics owned by application state.

mod migrations;

pub(crate) use migrations::run_migrations;
