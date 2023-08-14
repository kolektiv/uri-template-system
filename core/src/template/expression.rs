pub mod modifier;
pub mod operator;
pub mod variable_list;
pub mod variable_name;
pub mod variable_specification;

use anyhow::{
    Error,
    Result,
};

use crate::{
    template::expression::{
        operator::Operator,
        variable_list::VariableList,
    },
    value::Values,
    Expand,
    Parse,
    TryParse,
};

// =============================================================================
// Expression
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Expression<'a> {
    operator: Option<Operator<'a>>,
    raw: &'a str,
    variable_list: VariableList<'a>,
}

impl<'a> Expression<'a> {
    fn new(raw: &'a str, operator: Option<Operator<'a>>, variable_list: VariableList<'a>) -> Self {
        Self {
            operator,
            raw,
            variable_list,
        }
    }
}

impl<'a> TryParse<'a> for Expression<'a> {
    fn try_parse(raw: &'a str) -> Result<(usize, Self)> {
        let mut parsed_operator = None;
        let mut parsed_variable_list = Vec::new();
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Opening if raw[state.position..].starts_with('{') => {
                    state.next = Next::Operator;
                    state.position += 1;
                }
                Next::Opening => return Err(Error::msg("expr: expected opening brace")),
                Next::Operator => match Option::<Operator>::parse(&raw[state.position..]) {
                    (position, operator) => {
                        parsed_operator = operator;
                        state.next = Next::VariableList;
                        state.position += position;
                    }
                },
                Next::VariableList => match VariableList::try_parse(&raw[state.position..]) {
                    Ok((position, variable_list)) => {
                        parsed_variable_list.extend(variable_list);
                        state.next = Next::Closing;
                        state.position += position;
                    }
                    Err(err) => return Err(err),
                },
                Next::Closing if raw[state.position..].starts_with('}') => {
                    state.position += 1;

                    return Ok((
                        state.position,
                        Self::new(
                            &raw[..state.position],
                            parsed_operator,
                            parsed_variable_list,
                        ),
                    ));
                }
                Next::Closing => return Err(Error::msg("exp: expected closing brace")),
            }
        }
    }
}

#[derive(Default)]
struct State {
    next: Next,
    position: usize,
}

#[derive(Default)]
enum Next {
    #[default]
    Opening,
    Operator,
    VariableList,
    Closing,
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, ()> for Expression<'a> {
    fn expand(&self, output: &mut String, values: &Values, _: &()) {
        self.operator.expand(output, values, &self.variable_list);
    }
}
