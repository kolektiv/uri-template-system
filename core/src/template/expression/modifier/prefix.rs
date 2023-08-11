// =============================================================================
// Explode
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Prefix<'a> {
    length: usize,
    raw: &'a str,
}

impl<'a> Prefix<'a> {
    pub fn new(raw: &'a str, length: usize) -> Self {
        Self { length, raw }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
