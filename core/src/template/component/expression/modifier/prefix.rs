// =============================================================================
// Explode
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Prefix<'t> {
    length: usize,
    raw: &'t str,
}

impl<'t> Prefix<'t> {
    pub fn new(raw: &'t str, length: usize) -> Self {
        Self { length, raw }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
