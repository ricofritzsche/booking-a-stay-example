//! Book Stay capability.
//!
//! This module contains the Functional Core and its PostgreSQL-backed RPU shell.

pub mod context;
pub mod decide;
pub mod process;
pub mod request;
pub mod result;
pub mod sql;

#[cfg(test)]
mod decide_tests;
