mod parse;

// =================================================s============================
// Literal
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Literal(String);

impl Literal {
    #[allow(dead_code)]
    fn new(literal: impl Into<String>) -> Self {
        Self(literal.into())
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::parse::literal as parse;
