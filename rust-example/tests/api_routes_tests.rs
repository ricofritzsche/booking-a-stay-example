use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use booking_a_stay::{api, application_state::AppState, providers::Providers};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn post_api_reservations_is_routed() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/reservations")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{"))
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn old_stays_book_route_is_not_registered() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/stays/book")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{"))
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn health_remains_outside_api_namespace() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_health_is_not_registered() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

fn app() -> axum::Router {
    api::router(AppState::new(unreachable_pool(), Providers::new()))
}

fn unreachable_pool() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/booking_a_stay")
        .expect("test database URL should be valid")
}
