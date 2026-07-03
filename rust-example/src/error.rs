//! Application error strategy.
//!
//! Startup errors abort the process. Request errors implement Axum's
//! `IntoResponse` and become stable JSON responses. Business capabilities will
//! later return domain-specific rejections and map them into `ClientError`
//! variants at the API boundary.

// These types are part of the foundation before capability handlers exist.
#![allow(dead_code)]

use std::io;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

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

/// Request-time application errors.
#[derive(Debug, Error)]
pub enum AppError {
    /// Technical failure while processing a valid request.
    #[error(transparent)]
    Technical(#[from] TechnicalError),

    /// Client input problem or domain rejection.
    #[error(transparent)]
    Client(#[from] ClientError),
}

/// Technical request errors.
#[derive(Debug, Error)]
pub enum TechnicalError {
    /// Database operation failed.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    /// The request could not be completed because of an internal invariant.
    #[error("internal error: {0}")]
    Internal(&'static str),
}

/// Client-facing request errors and future domain rejections.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Request payload, query, or route parameters are invalid.
    #[error("{message}")]
    BadRequest {
        code: &'static str,
        message: &'static str,
    },

    /// Requested resource does not exist.
    #[error("{message}")]
    NotFound {
        code: &'static str,
        message: &'static str,
    },

    /// Request conflicts with current application state.
    #[error("{message}")]
    Conflict {
        code: &'static str,
        message: &'static str,
    },

    /// Business rule rejection from a capability.
    #[error("{message}")]
    DomainRejection {
        code: &'static str,
        message: &'static str,
    },
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Technical(TechnicalError::Database(_))
            | AppError::Technical(TechnicalError::Internal(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Client(ClientError::BadRequest { .. }) => StatusCode::BAD_REQUEST,
            AppError::Client(ClientError::NotFound { .. }) => StatusCode::NOT_FOUND,
            AppError::Client(ClientError::Conflict { .. })
            | AppError::Client(ClientError::DomainRejection { .. }) => StatusCode::CONFLICT,
        }
    }

    fn public_body(&self) -> ErrorBody {
        match self {
            AppError::Technical(TechnicalError::Database(_)) => ErrorBody {
                code: "database_error",
                message: "a database error occurred",
            },
            AppError::Technical(TechnicalError::Internal(_)) => ErrorBody {
                code: "internal_error",
                message: "an internal error occurred",
            },
            AppError::Client(ClientError::BadRequest { code, message })
            | AppError::Client(ClientError::NotFound { code, message })
            | AppError::Client(ClientError::Conflict { code, message })
            | AppError::Client(ClientError::DomainRejection { code, message }) => {
                ErrorBody { code, message }
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        if status.is_server_error() {
            error!(error = %self, status = status.as_u16(), "request failed");
        }

        let body = ErrorResponse {
            error: self.public_body(),
        };

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Technical(TechnicalError::Database(err))
    }
}
