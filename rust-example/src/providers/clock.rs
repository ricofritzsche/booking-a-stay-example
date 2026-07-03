//! Clock provider.

use chrono::{DateTime, Utc};

/// Source of the current time.
///
/// The shell reads the clock and passes the resulting timestamp *into* the
/// Functional Core as explicit data, so the core never reads time itself.
#[derive(Clone, Default)]
pub struct Clock;

impl Clock {
    /// Returns the current UTC time.
    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
