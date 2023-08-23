pub mod encode;
pub mod satisfy;

use std::fmt::Result;

// =============================================================================
// String
// =============================================================================

// Traits

pub trait Encode {
    fn encode(&mut self, raw: &str, satisfier: &impl Satisfy) -> Result;
}

pub trait Satisfy {
    fn satisfy(&self, input: &str) -> usize;
}
