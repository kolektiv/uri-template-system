mod parse;

use crate::{
    expression::Expression,
    literal::Literal,
};

// =============================================================================
// Template
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Template(Vec<Component>);

impl Template {
    #[allow(dead_code)]
    fn new(components: Vec<Component>) -> Self {
        Self(components)
    }
}

#[derive(Debug, PartialEq)]
enum Component {
    Expression(Expression),
    Literal(Literal),
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::parse::template as parse;
