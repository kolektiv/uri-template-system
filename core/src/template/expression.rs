mod modifier;
mod operator;
mod var_spec;

use nom::{
    character::complete as character,
    multi,
    sequence,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    value::Values,
    Expand,
};

// =============================================================================
// Expression
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Expression(Vec<VarSpec>, Option<Operator>);

impl Expression {
    fn new(variable_list: Vec<VarSpec>, operator: Option<Operator>) -> Self {
        Self(variable_list, operator)
    }
}

// -----------------------------------------------------------------------------

// Parsing

impl Expression {
    pub fn parse(input: &str) -> IResult<&str, Expression> {
        sequence::delimited(
            character::char('{'),
            Operator::parse
                .opt()
                .and(multi::separated_list1(character::char(','), VarSpec::parse)),
            character::char('}'),
        )
        .map(|(operator, variable_list)| Expression::new(variable_list, operator))
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, ()> for Expression {
    fn expand(&self, output: &mut String, values: &Values, _: &()) {
        self.1.expand(output, values, &self.0);
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::{
    modifier::*,
    operator::*,
    var_spec::*,
};
