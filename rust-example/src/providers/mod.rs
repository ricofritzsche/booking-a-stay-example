//! External providers.
//!
//! Thin adapters over dependencies that sit outside the decision logic: the
//! clock, id generation, and later things like payment services. Keeping these
//! behind explicit provider types lets the Imperative Shell inject time and ids
//! while the Functional Core stays pure and deterministic.
//!
//! These are deliberately minimal placeholders for the foundation step; not yet
//! consumed by any capability, so dead-code is allowed until `book_stay` exists.
#![allow(dead_code)]

mod bundle;
mod clock;
mod id_generator;

pub use bundle::Providers;
pub use clock::Clock;
pub use id_generator::IdGenerator;
