//! Application configuration.
//!
//! Configuration is assembled from the crate-local `config.toml` file and then
//! overlaid with environment variables prefixed `APP_` (e.g. `APP_SERVER__PORT`,
//! `APP_DATABASE__URL`). Nested keys use a double underscore as the separator.
//!
//! The foundation prefers explicit, typed configuration over ad-hoc `env::var`
//! lookups so that missing or malformed values fail fast at startup.

use serde::Deserialize;
use std::path::{Path, PathBuf};

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};

/// Fully resolved application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub telemetry: TelemetryConfig,
}

/// HTTP server binding.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Interface to bind, e.g. `0.0.0.0`.
    pub host: String,
    /// TCP port to listen on.
    pub port: u16,
}

/// PostgreSQL connection settings.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// Full connection string, e.g. `postgres://user:pass@host:5432/db`.
    pub url: String,
    /// Upper bound on pooled connections.
    pub max_connections: u32,
    /// Whether startup should apply SQLx migrations before serving traffic.
    #[serde(default = "default_run_migrations")]
    pub run_migrations: bool,
}

/// Logging and request tracing settings.
#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    /// Output format for application logs.
    pub log_format: LogFormat,
    /// Default filter used when neither `RUST_LOG` nor `APP_LOG_LEVEL` is set.
    pub log_level: String,
}

/// Supported log output formats.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Compact human-readable logs for local development.
    Pretty,
    /// Structured JSON logs for production log collectors.
    Json,
}

impl Config {
    /// Loads configuration from the crate-local `config.toml` and the environment.
    ///
    /// Environment variables win over file values, letting deployments override
    /// individual settings without editing files. The config file path is based
    /// on the crate manifest directory, so startup does not depend on the
    /// process working directory.
    pub fn load() -> Result<Self, crate::error::StartupError> {
        let config = Figment::new()
            .merge(Toml::file(default_config_path()))
            .merge(Env::prefixed("APP_").split("__"))
            .extract()?;
        validate(&config)?;
        Ok(config)
    }

    /// Convenience accessor for the socket address the server should bind.
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

fn default_config_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml")
}

fn default_run_migrations() -> bool {
    true
}

fn validate(config: &Config) -> Result<(), crate::error::StartupError> {
    if config.database.max_connections == 0 {
        return Err(crate::error::StartupError::InvalidConfig(
            "database.max_connections must be greater than 0",
        ));
    }

    Ok(())
}
