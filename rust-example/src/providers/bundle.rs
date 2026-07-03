//! Aggregate of all external providers.

use super::{Clock, IdGenerator};

/// Bundle of all external providers, cloned into [`crate::application_state::AppState`].
#[derive(Clone, Default)]
pub struct Providers {
    pub clock: Clock,
    pub ids: IdGenerator,
}

impl Providers {
    pub fn new() -> Self {
        Self::default()
    }
}
