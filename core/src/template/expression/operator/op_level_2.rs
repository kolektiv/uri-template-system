mod fragment;
mod reserved;

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
// OpLevel2
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub enum OpLevel2 {
    Reserved(Reserved),
    Fragment(Fragment),
}

// -----------------------------------------------------------------------------

// Parsing

impl OpLevel2 {
    pub fn parse(input: &str) -> IResult<&str, OpLevel2> {
        branch::alt((
            Fragment::parse.map(OpLevel2::Fragment),
            Reserved::parse.map(OpLevel2::Reserved),
        ))
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, Vec<VarSpec>> for OpLevel2 {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        match self {
            Self::Fragment(operator) => operator.expand(output, value, context),
            Self::Reserved(operator) => operator.expand(output, value, context),
        }
    }
}

impl Expand<Value, VarSpec> for OpLevel2 {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Self::Fragment(operator) => operator.expand(output, value, context),
            Self::Reserved(operator) => operator.expand(output, value, context),
        }
    }
}

impl Expand<String, VarSpec> for OpLevel2 {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        match self {
            Self::Fragment(operator) => operator.expand(output, value, context),
            Self::Reserved(operator) => operator.expand(output, value, context),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    fragment::*,
    reserved::*,
};
