mod op_level_2;
mod op_level_3;
mod op_reserve;

use nom::{
    branch,
    IResult,
    Parser,
};

use crate::{
    codec,
    template::{
        common,
        Modifier,
        Prefix,
        VarSpec,
    },
    value::{
        Value,
        Values,
    },
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
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        match self {
            Some(operator) => operator.expand(output, value, context),
            _ => {
                let mut defined = VarSpec::defined(context, value);

                while let Some((value, var_spec)) = defined.next() {
                    var_spec.expand(output, value, self);

                    if defined.peek().is_some() {
                        output.push(',');
                    }
                }
            }
        }
    }
}

impl Expand<Value, VarSpec> for Option<Operator> {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Some(operator) => operator.expand(output, value, context),
            _ => match value {
                Value::Item(value) => context.expand(output, value, &None),
                _ => todo!()
                // Value::List(value) => value
                //     .iter()
                //     .for_each(|value| codec::encode(value, output, unreserved())),
                // Value::AssociativeArray(value) => match context.1 {
                //     Some(Modifier::Explode) => value.iter().for_each(|(key, value)| {
                //         output.push_str(key);
                //         output.push('=');
                //         codec::encode(value, output, unreserved());
                //     }),
                //     _ => value.iter().for_each(|(key, value)| {
                //         output.push_str(key);
                //         output.push(',');
                //         codec::encode(value, output, unreserved());
                //     }),
                // },
            },
        }
    }
}

impl Expand<String, VarSpec> for Option<Operator> {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        match self {
            Some(operator) => operator.expand(output, value, context),
            _ => {
                let len = value.len();
                let len = match context.1 {
                    Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
                    _ => len,
                };

                codec::encode(&value[..len], output, common::unreserved());
            }
        }
    }
}

impl Expand<Values, Vec<VarSpec>> for Operator {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        match self {
            Self::Level2(operator) => operator.expand(output, value, context),
            Self::Level3(operator) => operator.expand(output, value, context),
            _ => unreachable!(),
        }
    }
}

impl Expand<Value, VarSpec> for Operator {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Self::Level2(operator) => operator.expand(output, value, context),
            Self::Level3(operator) => operator.expand(output, value, context),
            _ => unreachable!(),
        }
    }
}

impl Expand<String, VarSpec> for Operator {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        match self {
            Self::Level2(operator) => operator.expand(output, value, context),
            Self::Level3(operator) => operator.expand(output, value, context),
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
