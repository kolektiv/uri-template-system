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
// Fragment
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Fragment;

// -----------------------------------------------------------------------------

// Parsing

impl Fragment {
    pub fn parse(input: &str) -> IResult<&str, Fragment> {
        character::char('#').value(Fragment).parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

const FRAGMENT: Option<Operator> = Some(Operator::Level2(OpLevel2::Fragment(Fragment)));
const FRAGMENT_PREFIX: char = '#';
const FRAGMENT_INFIX: char = ',';

impl Expand<Values, Vec<VarSpec>> for Fragment {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        let mut defined = VarSpec::defined(context, value);

        if defined.peek().is_some() {
            output.push(FRAGMENT_PREFIX);
        }

        while let Some((value, var_spec)) = defined.next() {
            var_spec.expand(output, value, &FRAGMENT);

            if defined.peek().is_some() {
                output.push(FRAGMENT_INFIX);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Fragment {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.expand(output, value, &FRAGMENT),
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
                    output.push(FRAGMENT_INFIX);
                    codec::encode(value, output, common::reserved());
                }),
            },
        }
    }
}

impl Expand<String, VarSpec> for Fragment {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        let len = value.len();
        let len = match context.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&value[..len], output, common::reserved());
    }
}
