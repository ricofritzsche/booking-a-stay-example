//! Domain capabilities (Request Processing Units).
//!
//! Each capability lives in its own submodule here and owns its complete
//! request-processing path: request/response types, decision context,
//! Functional Core, Imperative Shell, and command-owned SQL.
//!
//! See the repository `guidelines/` for the required structure and implementation rules.

pub mod book_stay;
