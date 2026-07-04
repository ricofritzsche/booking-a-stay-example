//! HTTP route assembly.

use axum::{
    Router,
    routing::{get, post},
};

use super::{book_stay, health};
use crate::application_state::AppState;

/// Builds all HTTP routes for the application.
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .nest("/api", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new().route("/reservations", post(book_stay::handle))
}
