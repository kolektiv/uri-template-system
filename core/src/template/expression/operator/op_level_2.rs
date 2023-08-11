pub mod fragment;
pub mod reserved;

use crate::{
    template::{
        expression::operator::op_level_2::{
            fragment::Fragment,
            reserved::Reserved,
        },
        variable_list::VariableList,
    },
    value::Values,
    Expand,
};

// =============================================================================
// OpLevel2
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2<'a> {
    Fragment(Fragment<'a>),
    Reserved(Reserved<'a>),
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, VariableList<'a>> for OpLevel2<'a> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        match self {
            Self::Fragment(operator) => operator.expand(output, values, variable_list),
            Self::Reserved(operator) => operator.expand(output, values, variable_list),
        }
    }
}
