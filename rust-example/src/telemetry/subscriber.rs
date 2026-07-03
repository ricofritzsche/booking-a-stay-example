//! Tracing subscriber installation.

use std::env;

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Installs the process-wide tracing subscriber.
///
/// Safe to call at the very start of `main`; repeated calls are ignored so
/// tests can initialize app wiring without panicking. Verbosity follows
/// `RUST_LOG` (default `info`); set `APP_LOG_FORMAT=json` for structured
/// output.
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let use_json = env::var("APP_LOG_FORMAT").as_deref() == Ok("json");

    let registry = tracing_subscriber::registry().with(filter);

    if use_json {
        let _ = registry.with(fmt::layer().json()).try_init();
    } else {
        let _ = registry.with(fmt::layer()).try_init();
    }
}
