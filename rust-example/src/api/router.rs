//! Top-level router assembly.

use axum::{Router, routing::get};

use super::health;
use crate::application_state::AppState;

/// Builds the top-level application router.
///
/// Capability routes will be nested here as they are added.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .with_state(state)
}
