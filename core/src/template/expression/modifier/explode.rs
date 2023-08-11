// =============================================================================
// Explode
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Explode<'a> {
    raw: &'a str,
}

impl<'a> Explode<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self { raw }
    }
}
