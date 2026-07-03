//! Binary entrypoint.
//!
//! `main` stays intentionally tiny: it loads configuration, initializes
//! telemetry, and hands control to [`app::run`]. All wiring lives in [`app`],
//! all real work will live in capabilities.

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
    let config = config::Config::load()?;
    telemetry::init(&config.telemetry);

    app::run(config).await
}
