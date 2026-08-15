//! Book Stay capability.
//!
//! This module contains the Functional Core and its PostgreSQL-backed RPU shell.

pub mod context;
pub mod core;
pub mod process;
pub mod request;
pub mod result;
pub mod state_access;

#[cfg(test)]
mod core_tests;
