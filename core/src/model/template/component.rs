mod expression;
mod literal;

use std::fmt::Write;

use crate::{
    model::template::component::{
        expression::Expression,
        literal::Literal,
    },
    process::{
        expand::{
            Expand,
            ExpandError,
        },
        parse::{
            ParseError,
            TryParse,
        },
    },
    Values,
};

// =============================================================================
// Component
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'t> {
    Literal(Literal<'t>),
    Expression(Expression<'t>),
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Vec<Component<'t>> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut parsed_components = Self::new(); // TODO: Check if a default capacity estimation improves perf
        let mut state = ComponentState::default();

        loop {
            let rest = &raw[state.position..];

            if rest.is_empty() {
                break;
            }

            let parsed = if rest.starts_with('{') {
                Expression::try_parse(rest, global + state.position)
                    .map(|(position, expression)| (position, Component::Expression(expression)))
            } else {
                Literal::try_parse(rest, global + state.position)
                    .map(|(position, literal)| (position, Component::Literal(literal)))
            };

            match parsed {
                Ok((position, component)) => {
                    parsed_components.push(component);
                    state.position += position;
                }
                Err(err) => return Err(err),
            }
        }

        Ok((raw.len(), parsed_components))
    }
}

#[derive(Default)]
struct ComponentState {
    position: usize,
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Component<'t> {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        match self {
            Self::Expression(expression) => expression.expand(values, write),
            Self::Literal(literal) => literal.expand(values, write),
        }
    }
}
