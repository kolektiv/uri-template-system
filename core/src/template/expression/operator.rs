mod none;
mod op_level_2;
mod op_level_3;
mod op_reserve;

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
// Operator
// =============================================================================

// Types

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
    Reserve(OpReserve),
}

// -----------------------------------------------------------------------------

// Parsing

impl Operator {
    pub fn parse(input: &str) -> IResult<&str, Operator> {
        branch::alt((
            OpLevel2::parse.map(Operator::Level2),
            OpLevel3::parse.map(Operator::Level3),
        ))
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, Vec<VarSpec>> for Option<Operator> {
    fn expand(&self, output: &mut String, values: &Values, var_specs: &Vec<VarSpec>) {
        match self {
            Some(operator) => operator.expand(output, values, var_specs),
            _ => none::None.expand(output, values, var_specs),
        }
    }
}

impl Expand<Values, Vec<VarSpec>> for Operator {
    fn expand(&self, output: &mut String, values: &Values, var_specs: &Vec<VarSpec>) {
        match self {
            Self::Level2(operator) => operator.expand(output, values, var_specs),
            Self::Level3(operator) => operator.expand(output, values, var_specs),
            _ => unreachable!(),
        }
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    op_level_2::*,
    op_level_3::*,
    op_reserve::*,
};
