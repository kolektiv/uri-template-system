use crate::ParseRef;

// =============================================================================
// Explode
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Explode<'a> {
    parse_ref: ParseRef<'a>,
}

impl<'a> Explode<'a> {
    pub fn new(parse_ref: ParseRef<'a>) -> Self {
        Self { parse_ref }
    }
}
