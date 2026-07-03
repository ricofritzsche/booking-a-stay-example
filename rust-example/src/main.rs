//! Binary entrypoint.
//!
//! `main` stays intentionally tiny: it initializes telemetry, loads
//! configuration, and hands control to [`app::run`]. All wiring lives in
//! [`app`], all real work will live in capabilities.

mod api;
mod app;
mod application_state;
mod capabilities;
mod config;
mod db;
mod error;
mod providers;
mod telemetry;

use crate::error::StartupError;

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    // Structured logging must be available before anything else runs so that
    // configuration and startup errors are captured.
    telemetry::init();

    let config = config::Config::load()?;

    app::run(config).await
}
