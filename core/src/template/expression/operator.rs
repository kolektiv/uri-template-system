mod none;
mod op_level_2;
mod op_level_3;
mod op_reserve;

use anyhow::Result;

use crate::{
    template::expression::{
        operator::{
            op_level_2::{
                fragment::Fragment,
                reserved::Reserved,
                OpLevel2,
            },
            op_level_3::{
                label::Label,
                path::Path,
                path_parameter::PathParameter,
                query::Query,
                query_continuation::QueryContinuation,
                OpLevel3,
            },
        },
        variable_list::VariableList,
    },
    value::Values,
    Expand,
    Parse,
    ParseRef,
};

// =============================================================================
// Operator
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum Operator<'a> {
    Level2(OpLevel2<'a>),
    Level3(OpLevel3<'a>),
}

#[rustfmt::skip]
impl<'a> Parse<'a> for Option<Operator<'a>> {
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
        Ok(raw.chars().next().and_then(|c| {
            let len = 1;
            let parse_ref = ParseRef::new(base, base, &raw[..1]);
            let operator = match c {
                '+' => Some(Operator::Level2(OpLevel2::Reserved(Reserved::new(parse_ref)))),
                '#' => Some(Operator::Level2(OpLevel2::Fragment(Fragment::new(parse_ref)))),
                '.' => Some(Operator::Level3(OpLevel3::Label(Label::new(parse_ref)))),
                '/' => Some(Operator::Level3(OpLevel3::Path(Path::new(parse_ref)))),
                ';' => Some(Operator::Level3(OpLevel3::PathParameter(PathParameter::new(parse_ref)))),
                '?' => Some(Operator::Level3(OpLevel3::Query(Query::new(parse_ref)))),
                '&' => Some(Operator::Level3(OpLevel3::QueryContinuation(QueryContinuation::new(parse_ref)))),
                _ => None,
            };

            operator.map(|operator| (len, Some(operator)))
        })
        .unwrap_or((0, None)))
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, VariableList<'a>> for Option<Operator<'a>> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        match self {
            Some(operator) => operator.expand(output, values, variable_list),
            _ => none::None.expand(output, values, variable_list),
        }
    }
}

impl<'a> Expand<Values, VariableList<'a>> for Operator<'a> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        match self {
            Self::Level2(operator) => operator.expand(output, values, variable_list),
            Self::Level3(operator) => operator.expand(output, values, variable_list),
        }
    }
}
