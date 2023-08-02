mod common;
mod expression;
mod literal;

use nom::{
    branch,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
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

#[derive(Debug, PartialEq)]
enum Component {
    Expression(Expression),
    Literal(Literal),
}

// -----------------------------------------------------------------------------

// Parsing

impl Template {
    pub fn parse(input: &str) -> IResult<&str, Template> {
        multi::many1(branch::alt((
            Literal::parse.map(Component::Literal),
            Expression::parse.map(Component::Expression),
        )))
        .all_consuming()
        .map(Template)
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, ()> for Template {
    fn expand(&self, output: &mut String, values: &Values, context: &()) {
        self.0
            .iter()
            .for_each(|component| component.expand(output, values, context));
    }
}

impl Expand<Values, ()> for Component {
    fn expand(&self, output: &mut String, values: &Values, context: &()) {
        match self {
            Component::Expression(expression) => expression.expand(output, values, context),
            Component::Literal(literal) => literal.expand(output, values, context),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    expression::*,
    literal::*,
};
