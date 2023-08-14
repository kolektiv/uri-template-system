use anyhow::Result;

use crate::{
    template::{
        expression::Expression,
        literal::Literal,
    },
    value::Values,
    Expand,
    TryParse,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'a> {
    Literal(Literal<'a>),
    Expression(Expression<'a>),
}

// Parse

impl<'a> TryParse<'a> for Vec<Component<'a>> {
    fn try_parse(raw: &'a str) -> Result<(usize, Self)> {
        let mut parsed_components = Self::new(); // TODO: Check if a default capacity estimation improves perf
        let mut state = State::default();

        loop {
            if state.position >= raw.len() {
                break;
            }

            let parsed = if raw[state.position..].starts_with('{') {
                Expression::try_parse(&raw[state.position..])
                    .map(|(cursor, expression)| (cursor, Component::Expression(expression)))
            } else {
                Literal::try_parse(&raw[state.position..])
                    .map(|(cursor, literal)| (cursor, Component::Literal(literal)))
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
struct State {
    position: usize,
}

// Expand

impl<'a> Expand<Values, ()> for Component<'a> {
    fn expand(&self, output: &mut String, values: &Values, context: &()) {
        match self {
            Component::Expression(expression) => expression.expand(output, values, context),
            Component::Literal(literal) => literal.expand(output, values, context),
        }
    }
}
