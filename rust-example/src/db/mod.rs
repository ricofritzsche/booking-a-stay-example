//! Database connectivity.
//!
//! Owns the mechanics of establishing a PostgreSQL connection pool. This module
//! holds no domain meaning and no queries — capabilities own their own SQL
//! (see the architecture guidelines).

mod connection;

pub use connection::connect;
