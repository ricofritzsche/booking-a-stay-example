use std::time::Duration;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use booking_a_stay::{api, application_state::AppState, providers::Providers};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_service_unavailable_when_database_is_unreachable() {
    let app = api::router(AppState::new(unreachable_pool(), Providers::new()));

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

fn unreachable_pool() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/booking_a_stay")
        .expect("test database URL should be valid")
}
