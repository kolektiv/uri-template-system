mod parse;

use crate::{
    expression::Expression,
    literal::Literal,
    value::Values,
    Expand,
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

impl Expand<Values, ()> for Template {
    fn expand(&self, output: &mut String, value: &Values, context: &()) {
        self.0
            .iter()
            .for_each(|component| component.expand(output, value, context));
    }
}

#[derive(Debug, PartialEq)]
enum Component {
    Expression(Expression),
    Literal(Literal),
}

impl Expand<Values, ()> for Component {
    fn expand(&self, output: &mut String, value: &Values, context: &()) {
        match self {
            Component::Expression(expression) => expression.expand(output, value, context),
            Component::Literal(literal) => literal.expand(output, value, context),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::parse::template as parse;
