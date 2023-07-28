mod parse;

use anyhow::Result;

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

impl Expand for Template {
    type Context = ();

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        for component in self.0.iter() {
            component.expand(output, values, context)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Component {
    Expression(Expression),
    Literal(Literal),
}

impl Expand for Component {
    type Context = ();

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        match self {
            Component::Expression(expression) => expression.expand(output, values, context),
            Component::Literal(literal) => literal.expand(output, values, context),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::parse::template as parse;
