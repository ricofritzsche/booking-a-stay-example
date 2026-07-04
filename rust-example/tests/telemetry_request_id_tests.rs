use std::time::Duration;

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::State,
    http::{Request, StatusCode},
    routing::get,
};
use booking_a_stay::{
    api,
    application_state::AppState,
    providers::Providers,
    telemetry::{self, RequestId},
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn response_propagates_request_id() {
    let app = telemetry::http_layer(
        api::router(AppState::new(unreachable_pool(), Providers::new())),
        Duration::from_secs(1),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("x-request-id", "test-request-id")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(
        response.headers().get("x-request-id").unwrap(),
        "test-request-id"
    );
}

#[tokio::test]
async fn response_generates_request_id_when_missing() {
    let app = telemetry::http_layer(
        api::router(AppState::new(unreachable_pool(), Providers::new())),
        Duration::from_secs(1),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert!(response.headers().get("x-request-id").is_some());
}

#[tokio::test]
async fn request_id_is_available_inside_handlers() {
    #[derive(Serialize)]
    struct RequestIdResponse {
        request_id: String,
    }

    async fn echo_request_id(
        State(_state): State<AppState>,
        request_id: RequestId,
    ) -> Json<RequestIdResponse> {
        Json(RequestIdResponse {
            request_id: request_id.as_str().to_owned(),
        })
    }

    let app = telemetry::http_layer(
        Router::new()
            .route("/request-id", get(echo_request_id))
            .with_state(AppState::new(unreachable_pool(), Providers::new())),
        Duration::from_secs(1),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/request-id")
                .header("x-request-id", "handler-request-id")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let body = std::str::from_utf8(&body).expect("response body should be UTF-8");

    assert!(body.contains(r#""request_id":"handler-request-id""#));
}

fn unreachable_pool() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/booking_a_stay")
        .expect("test database URL should be valid")
}
