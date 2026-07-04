//! Health check endpoint.
//!
//! Reports process liveness and database readiness. Kept in the API layer
//! because it is an operational concern, not a Domain Capability.

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::application_state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

/// `GET /health` — returns 200 when the database is reachable, 503 otherwise.
pub async fn health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => (
            StatusCode::OK,
            Json(HealthResponse {
                status: "ok",
                database: "up",
            }),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "degraded",
                database: "down",
            }),
        ),
    }
}
