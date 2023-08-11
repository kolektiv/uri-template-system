pub mod label;
pub mod path;
pub mod path_parameter;
pub mod query;
pub mod query_continuation;

use crate::{
    template::expression::{
        operator::op_level_3::{
            label::Label,
            path::Path,
            path_parameter::PathParameter,
            query::Query,
            query_continuation::QueryContinuation,
        },
        variable_list::VariableList,
    },
    value::Values,
    Expand,
};

// =============================================================================
// OpLevel3
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3<'a> {
    Label(Label<'a>),
    Path(Path<'a>),
    PathParameter(PathParameter<'a>),
    Query(Query<'a>),
    QueryContinuation(QueryContinuation<'a>),
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, VariableList<'a>> for OpLevel3<'a> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        match self {
            Self::Label(operator) => operator.expand(output, values, variable_list),
            Self::Path(operator) => operator.expand(output, values, variable_list),
            Self::PathParameter(operator) => operator.expand(output, values, variable_list),
            Self::Query(operator) => operator.expand(output, values, variable_list),
            Self::QueryContinuation(operator) => operator.expand(output, values, variable_list),
        }
    }
}
