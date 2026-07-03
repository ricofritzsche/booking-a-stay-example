//! Foundation-level error types.
//!
//! These describe failures that can occur while *starting or running the
//! process itself* — not domain rejections. Business errors belong inside each
//! capability and must never surface here. We use explicit `thiserror` enums
//! rather than `anyhow` so the foundation's failure modes stay visible.

use std::io;

use thiserror::Error;

/// Errors that can abort startup or the running server.
#[derive(Debug, Error)]
pub enum StartupError {
    /// Configuration could not be loaded or was invalid.
    ///
    /// Boxed because `figment::Error` is large and would otherwise bloat every
    /// `Result<_, StartupError>` on the startup path.
    #[error("failed to load configuration: {0}")]
    Config(Box<figment::Error>),

    /// Configuration loaded successfully but failed application validation.
    #[error("invalid configuration: {0}")]
    InvalidConfig(&'static str),

    /// The database pool could not be established.
    #[error("failed to initialize database: {0}")]
    Database(#[from] sqlx::Error),

    /// The server socket could not be bound or the server exited abnormally.
    #[error("server error: {0}")]
    Server(#[from] io::Error),
}

impl From<figment::Error> for StartupError {
    fn from(err: figment::Error) -> Self {
        StartupError::Config(Box::new(err))
    }
}
