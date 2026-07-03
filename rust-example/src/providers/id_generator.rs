//! Identifier provider.

use uuid::Uuid;

/// Source of new identifiers.
#[derive(Clone, Default)]
pub struct IdGenerator;

impl IdGenerator {
    /// Generates a fresh v4 UUID.
    pub fn new_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}
