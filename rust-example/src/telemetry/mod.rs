//! Telemetry setup.
//!
//! Initializes the global `tracing` subscriber. Output is human-readable by
//! default; set `APP_LOG_FORMAT=json` for structured logs in production.

mod subscriber;

pub use subscriber::init;
