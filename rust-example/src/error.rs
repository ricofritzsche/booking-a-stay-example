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
use tracing::{error, warn};

use crate::telemetry::RequestId;

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

/// Request-time application error with optional request context.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct AppError {
    kind: AppErrorKind,
    request_id: Option<RequestId>,
}

impl AppError {
    pub fn new(kind: impl Into<AppErrorKind>) -> Self {
        Self {
            kind: kind.into(),
            request_id: None,
        }
    }

    /// Attaches the current request ID so logs and error responses can be
    /// correlated with the request span.
    pub fn with_request_id(mut self, request_id: RequestId) -> Self {
        self.request_id = Some(request_id);
        self
    }

    fn status_code(&self) -> StatusCode {
        self.kind.status_code()
    }

    fn public_body(&self) -> ErrorBody {
        self.kind.public_body(self.request_id.as_ref())
    }
}

/// Request-time error categories.
#[derive(Debug, Error)]
pub enum AppErrorKind {
    /// Technical failure while processing a valid request.
    #[error(transparent)]
    Technical(#[from] TechnicalError),

    /// Client input problem or domain rejection.
    #[error(transparent)]
    Client(#[from] ClientError),
}

impl AppErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            AppErrorKind::Technical(TechnicalError::Database(_))
            | AppErrorKind::Technical(TechnicalError::Internal(_)) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppErrorKind::Client(ClientError::BadRequest { .. }) => StatusCode::BAD_REQUEST,
            AppErrorKind::Client(ClientError::NotFound { .. }) => StatusCode::NOT_FOUND,
            AppErrorKind::Client(ClientError::Conflict { .. })
            | AppErrorKind::Client(ClientError::Domain(_)) => StatusCode::CONFLICT,
        }
    }

    fn public_body(&self, request_id: Option<&RequestId>) -> ErrorBody {
        let (code, message) = match self {
            AppErrorKind::Technical(TechnicalError::Database(_)) => {
                ("database_error", "a database error occurred")
            }
            AppErrorKind::Technical(TechnicalError::Internal(_)) => {
                ("internal_error", "an internal error occurred")
            }
            AppErrorKind::Client(ClientError::BadRequest { code, message })
            | AppErrorKind::Client(ClientError::NotFound { code, message })
            | AppErrorKind::Client(ClientError::Conflict { code, message }) => (*code, *message),
            AppErrorKind::Client(ClientError::Domain(rejection)) => {
                (rejection.code(), rejection.message())
            }
        };

        ErrorBody {
            code,
            message,
            request_id: request_id.map(|value| value.as_str().to_owned()),
        }
    }
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
    #[error(transparent)]
    Domain(#[from] DomainRejection),
}

/// Business rejection returned by a capability.
///
/// Capabilities can keep their own rich rejection enums internally and convert
/// their public rejection result into this small transport-neutral shape at the
/// shell/API boundary.
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct DomainRejection {
    code: &'static str,
    message: &'static str,
}

impl DomainRejection {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let request_id = self.request_id.as_ref().map(RequestId::as_str);

        if status.is_server_error() {
            error!(
                error = %self.kind,
                status = status.as_u16(),
                request_id,
                "request failed",
            );
        } else if matches!(self.kind, AppErrorKind::Client(ClientError::Domain(_))) {
            warn!(
                error = %self.kind,
                status = status.as_u16(),
                request_id,
                "request rejected by domain",
            );
        }

        let body = ErrorResponse {
            error: self.public_body(),
        };

        (status, Json(body)).into_response()
    }
}

impl From<AppErrorKind> for AppError {
    fn from(kind: AppErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<TechnicalError> for AppError {
    fn from(err: TechnicalError) -> Self {
        Self::new(err)
    }
}

impl From<ClientError> for AppError {
    fn from(err: ClientError) -> Self {
        Self::new(err)
    }
}

impl From<DomainRejection> for AppError {
    fn from(err: DomainRejection) -> Self {
        Self::new(ClientError::Domain(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::new(TechnicalError::Database(err))
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};

    use super::{AppError, DomainRejection, TechnicalError};
    use crate::telemetry::RequestId;

    #[tokio::test]
    async fn domain_rejection_is_client_error_with_request_id() {
        let error = AppError::from(DomainRejection::new(
            "listing_unavailable",
            "listing is unavailable for the requested dates",
        ))
        .with_request_id(RequestId::from("request-123"));

        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);

        let body = response_body(response).await;
        assert!(body.contains(r#""code":"listing_unavailable""#));
        assert!(body.contains(r#""request_id":"request-123""#));
    }

    #[tokio::test]
    async fn technical_error_is_internal_error_without_private_details() {
        let error = AppError::from(TechnicalError::Internal("database password leaked here"))
            .with_request_id(RequestId::from("request-456"));

        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = response_body(response).await;
        assert!(body.contains(r#""code":"internal_error""#));
        assert!(body.contains(r#""message":"an internal error occurred""#));
        assert!(body.contains(r#""request_id":"request-456""#));
        assert!(!body.contains("database password leaked here"));
    }

    async fn response_body(response: axum::response::Response) -> String {
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");

        std::str::from_utf8(&body)
            .expect("response body should be UTF-8")
            .to_owned()
    }
}
