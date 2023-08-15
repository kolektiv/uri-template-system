// =============================================================================
// Explode
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Explode<'t> {
    raw: &'t str,
}

impl<'t> Explode<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}
