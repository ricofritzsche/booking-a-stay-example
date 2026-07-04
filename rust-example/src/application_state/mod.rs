//! Shared application state.
//!
//! Holds the infrastructure handles that capabilities need at runtime — the
//! PostgreSQL pool and the external providers. This is shared *mechanics*, not
//! shared domain meaning: it must never contain business rules, entities, or
//! decision logic.

mod app_state;
pub(crate) mod db;

pub use app_state::AppState;
