mod label;
mod path;
mod path_parameter;
mod query;
mod query_continuation;

use nom::{
    branch,
    IResult,
    Parser,
};

use crate::{
    template::VarSpec,
    value::{
        Value,
        Values,
    },
    Expand,
};

// =============================================================================
// OpLevel3
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub enum OpLevel3 {
    Label(Label),
    Path(Path),
    PathParameter(PathParameter),
    Query(Query),
    QueryContinuation(QueryContinuation),
}

// -----------------------------------------------------------------------------

// Parsing

impl OpLevel3 {
    pub fn parse(input: &str) -> IResult<&str, OpLevel3> {
        branch::alt((
            Label::parse.map(OpLevel3::Label),
            Path::parse.map(OpLevel3::Path),
            PathParameter::parse.map(OpLevel3::PathParameter),
            Query::parse.map(OpLevel3::Query),
            QueryContinuation::parse.map(OpLevel3::QueryContinuation),
        ))
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, Vec<VarSpec>> for OpLevel3 {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        match self {
            Self::Label(operator) => operator.expand(output, value, context),
            Self::Path(operator) => operator.expand(output, value, context),
            _ => todo!()
            // Self::PathParameter(operator) => operator.expand(output, value, context),
            // Self::Query(operator) => operator.expand(output, value, context),
            // Self::QueryContinuation(operator) => operator.expand(output, value, context),
        }
    }
}

impl Expand<Value, VarSpec> for OpLevel3 {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Self::Label(operator) => operator.expand(output, value, context),
            Self::Path(operator) => operator.expand(output, value, context),
            _ => todo!()
            // Self::Path(operator) => operator.expand(output, value, context),
            // Self::PathParameter(operator) => operator.expand(output, value, context),
            // Self::Query(operator) => operator.expand(output, value, context),
            // Self::QueryContinuation(operator) => operator.expand(output, value, context),
        }
    }
}

impl Expand<String, VarSpec> for OpLevel3 {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        match self {
            Self::Label(operator) => operator.expand(output, value, context),
            Self::Path(operator) => operator.expand(output, value, context),
            _ => todo!()
            // Self::Path(operator) => operator.expand(output, value, context),
            // Self::PathParameter(operator) => operator.expand(output, value, context),
            // Self::Query(operator) => operator.expand(output, value, context),
            // Self::QueryContinuation(operator) => operator.expand(output, value, context),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    label::*,
    path::*,
    path_parameter::*,
    query::*,
    query_continuation::*,
};
