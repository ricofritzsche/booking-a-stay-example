//! Top-level router assembly.

use axum::Router;

use super::http;
use crate::application_state::AppState;

/// Builds the top-level application router.
///
/// Capability routes will be nested here as they are added.
pub fn router(state: AppState) -> Router {
    http::routes(state)
}
