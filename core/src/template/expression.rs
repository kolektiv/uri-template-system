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
    ParseRef,
};

// =============================================================================
// Expression
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Expression<'a> {
    operator: Option<Operator<'a>>,
    parse_ref: ParseRef<'a>,
    variable_list: VariableList<'a>,
}

impl<'a> Expression<'a> {
    fn new(
        parse_ref: ParseRef<'a>,
        operator: Option<Operator<'a>>,
        variable_list: VariableList<'a>,
    ) -> Self {
        Self {
            operator,
            parse_ref,
            variable_list,
        }
    }
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
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
                Next::Operator => {
                    match Option::<Operator>::parse(&raw[state.position..], base + state.position) {
                        Ok((position, operator)) => {
                            parsed_operator = operator;
                            state.next = Next::VariableList;
                            state.position += position;
                        }
                        Err(err) => return Err(err),
                    }
                }
                Next::VariableList => {
                    match VariableList::parse(&raw[state.position..], base + state.position) {
                        Ok((position, variable_list)) => {
                            parsed_variable_list.extend(variable_list);
                            state.next = Next::Closing;
                            state.position += position;
                        }
                        Err(err) => return Err(err),
                    }
                }
                Next::Closing if raw[state.position..].starts_with('}') => {
                    state.position += 1;

                    let len = state.position;
                    let parse_ref = ParseRef::new(base, base + len - 1, &raw[..len]);

                    return Ok((
                        len,
                        Self::new(parse_ref, parsed_operator, parsed_variable_list),
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
