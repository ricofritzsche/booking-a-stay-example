//! Telemetry setup.
//!
//! Initializes the global `tracing` subscriber and owns HTTP observability
//! middleware: request IDs, spans, status codes, and latency logging.

mod subscriber;

pub use subscriber::{RequestId, http_layer, init};
