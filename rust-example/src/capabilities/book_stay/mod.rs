//! Book Stay capability.
//!
//! This module currently contains only the Functional Core: explicit input
//! data, decision context, and decision result. The RPU shell, SQL, locking,
//! and API delivery are intentionally out of scope for this step.

pub mod context;
pub mod decide;
pub mod request;
pub mod result;

#[cfg(test)]
mod decide_tests;
