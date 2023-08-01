use nom::{
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    codec,
    template::{
        common,
        Modifier,
        OpLevel2,
        Operator,
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
// Reserved
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Reserved;

// -----------------------------------------------------------------------------

// Parsing

impl Reserved {
    pub fn parse(input: &str) -> IResult<&str, Reserved> {
        character::char('+').value(Reserved).parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

const RESERVED: Option<Operator> = Some(Operator::Level2(OpLevel2::Reserved(Reserved)));
const RESERVED_INFIX: char = ',';

impl Expand<Values, Vec<VarSpec>> for Reserved {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        let mut defined = VarSpec::defined(context, value);

        while let Some((value, var_spec)) = defined.next() {
            var_spec.expand(output, value, &RESERVED);

            if defined.peek().is_some() {
                output.push(RESERVED_INFIX);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.expand(output, value, &RESERVED),
            Value::List(value) => value
                .iter()
                .for_each(|value| codec::encode(value, output, common::reserved())),
            Value::AssociativeArray(value) => match context.1 {
                Some(Modifier::Explode(_)) => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push('=');
                    codec::encode(value, output, common::reserved());
                }),
                _ => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push(RESERVED_INFIX);
                    codec::encode(value, output, common::reserved());
                }),
            },
        }
    }
}

impl Expand<String, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        let len = value.len();
        let len = match context.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&value[..len], output, common::reserved());
    }
}
