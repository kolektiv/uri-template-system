use crate::ParseRef;

// =============================================================================
// Explode
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Prefix<'a> {
    length: usize,
    parse_ref: ParseRef<'a>,
}

impl<'a> Prefix<'a> {
    pub fn new(parse_ref: ParseRef<'a>, length: usize) -> Self {
        Self { length, parse_ref }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
