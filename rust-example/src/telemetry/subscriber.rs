//! Tracing subscriber and HTTP observability middleware.

use std::{env, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::FromRequestParts,
    http::{Request, StatusCode, request::Parts},
};
use tower::ServiceBuilder;
use tower_http::{
    ServiceBuilderExt,
    request_id::{MakeRequestUuid, RequestId as TowerRequestId},
    timeout::TimeoutLayer,
    trace::{DefaultOnFailure, TraceLayer},
};
use tracing::{Level, Span, field, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config::{LogFormat, TelemetryConfig};

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Request correlation ID for logs, responses, and capability execution.
///
/// The HTTP middleware guarantees every request has an `x-request-id` header.
/// Handlers can extract this type and pass the plain string into capability
/// shells when they want capability logs to share the same correlation ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(String);

impl RequestId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &Parts) -> Option<Self> {
        parts
            .extensions
            .get::<TowerRequestId>()
            .and_then(Self::from_tower_request_id)
            .or_else(|| {
                parts
                    .headers
                    .get(REQUEST_ID_HEADER)
                    .and_then(|value| value.to_str().ok())
                    .map(Self::from)
            })
    }

    fn from_tower_request_id(request_id: &TowerRequestId) -> Option<Self> {
        request_id.header_value().to_str().ok().map(Self::from)
    }
}

impl From<&str> for RequestId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for RequestId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::from_parts(parts).ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Installs the process-wide tracing subscriber.
///
/// `RUST_LOG` wins when set. `APP_LOG_LEVEL` is a lightweight application
/// fallback, and the config file provides the final default. Repeated calls are
/// ignored so tests can initialize app wiring without panicking.
pub fn init(config: &TelemetryConfig) {
    let filter = log_filter(config);
    let format = log_format(config);
    let registry = tracing_subscriber::registry().with(filter);

    match format {
        LogFormat::Pretty => {
            let _ = registry
                .with(fmt::layer().compact().with_target(true))
                .try_init();
        }
        LogFormat::Json => {
            let _ = registry.with(fmt::layer().json()).try_init();
        }
    }
}

/// Applies HTTP request observability to the application router.
///
/// The order is important: request IDs are set before `TraceLayer` creates the
/// request span, and propagated after the response has been produced.
pub fn http_layer<S>(router: Router<S>, request_timeout: Duration) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(
        ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_request_span)
                    .on_response(record_response)
                    .on_failure(DefaultOnFailure::new().level(Level::WARN)),
            )
            .propagate_x_request_id()
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                request_timeout,
            )),
    )
}

fn log_filter(config: &TelemetryConfig) -> EnvFilter {
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        return filter;
    }

    let level = env::var("APP_LOG_LEVEL").unwrap_or_else(|_| config.log_level.clone());

    EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"))
}

fn log_format(config: &TelemetryConfig) -> LogFormat {
    env::var("APP_LOG_FORMAT")
        .ok()
        .and_then(|value| match value.as_str() {
            "pretty" | "human" | "text" => Some(LogFormat::Pretty),
            "json" => Some(LogFormat::Json),
            _ => None,
        })
        .unwrap_or(config.log_format)
}

fn make_request_span(request: &Request<Body>) -> Span {
    let request_id = request
        .extensions()
        .get::<TowerRequestId>()
        .and_then(RequestId::from_tower_request_id)
        .map(|value| value.0)
        .or_else(|| {
            request
                .headers()
                .get(REQUEST_ID_HEADER)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "unknown".to_owned());

    tracing::info_span!(
        "http_request",
        method = %request.method(),
        path = %request.uri().path(),
        status = field::Empty,
        latency_ms = field::Empty,
        request_id = %request_id,
    )
}

fn record_response<B>(response: &axum::http::Response<B>, latency: Duration, span: &Span) {
    let status = response.status().as_u16();
    let latency_ms = latency.as_millis() as u64;

    span.record("status", status);
    span.record("latency_ms", latency_ms);

    info!(
        parent: span,
        status,
        latency_ms,
        "request completed",
    );
}
