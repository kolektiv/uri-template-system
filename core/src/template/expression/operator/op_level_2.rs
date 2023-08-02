mod fragment;
mod reserved;

use nom::{
    branch,
    IResult,
    Parser,
};

use crate::{
    template::VarSpec,
    value::Values,
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
    fn expand(&self, output: &mut String, values: &Values, var_specs: &Vec<VarSpec>) {
        match self {
            Self::Fragment(operator) => operator.expand(output, values, var_specs),
            Self::Reserved(operator) => operator.expand(output, values, var_specs),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    fragment::*,
    reserved::*,
};
