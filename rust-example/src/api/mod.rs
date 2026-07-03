//! HTTP delivery layer.
//!
//! Thin translation between the outside world and capabilities. It assembles
//! the Axum router and adapts requests/responses. It must stay minimal and
//! contain no business logic — handlers delegate to capabilities.

mod health;
mod router;

pub use router::router;
