//! Shared runtime state.

use sqlx::PgPool;

use crate::providers::Providers;

/// Runtime state cloned into each request handler.
///
/// `PgPool` and the provider handles are cheaply cloneable (internally
/// reference-counted), so `AppState` is `Clone` and can be used directly as
/// Axum shared state.
#[derive(Clone)]
pub struct AppState {
    /// Connection pool backing the relational Application State.
    pub pool: PgPool,
    /// External dependencies (clock, id generation, ...).
    // Consumed by capabilities once they exist; unused in the foundation.
    #[allow(dead_code)]
    pub providers: Providers,
}

impl AppState {
    pub fn new(pool: PgPool, providers: Providers) -> Self {
        Self { pool, providers }
    }
}
