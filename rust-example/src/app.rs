//! Application composition and lifecycle.
//!
//! Responsible for wiring the pieces together (config -> pool -> providers ->
//! state -> router) and running the HTTP server with graceful shutdown. This is
//! the one place that knows how the whole application fits together; `main`
//! only calls [`run`].

use std::time::Duration;

use axum::http::StatusCode;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::application_state::AppState;
use crate::config::Config;
use crate::error::StartupError;
use crate::providers::Providers;
use crate::{api, db};

/// Builds and runs the application until a shutdown signal is received.
pub async fn run(config: Config) -> Result<(), StartupError> {
    let pool = db::connect(&config.database).await?;
    info!("database pool established");

    let state = AppState::new(pool, Providers::new());

    let app = api::router(state)
        // Emit a span + structured log per request.
        .layer(TraceLayer::new_for_http())
        // Bound how long any single request may run; reply 408 on timeout.
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ));

    let listener = TcpListener::bind(config.bind_address()).await?;
    info!(address = %config.bind_address(), "listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("shutdown complete");
    Ok(())
}

/// Resolves when the process receives Ctrl-C or (on Unix) SIGTERM.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received");
}
