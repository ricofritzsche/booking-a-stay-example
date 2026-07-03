//! Health check endpoint.
//!
//! Reports process liveness and database readiness. Kept in the delivery layer
//! because it is an operational concern, not a domain capability.

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    use crate::{api, application_state::AppState, providers::Providers};

    #[tokio::test]
    async fn health_returns_service_unavailable_when_database_is_unreachable() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(50))
            .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/booking_a_stay")
            .expect("test database URL should be valid");
        let app = api::router(AppState::new(pool, Providers::new()));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request should be valid"),
            )
            .await
            .expect("router should produce a response");

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let body = std::str::from_utf8(&body).expect("response body should be UTF-8");

        assert!(body.contains(r#""status":"degraded""#));
        assert!(body.contains(r#""database":"down""#));
    }
}
