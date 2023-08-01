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
        OpLevel3,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Path;

// -----------------------------------------------------------------------------

// Parsing

impl Path {
    pub fn parse(input: &str) -> IResult<&str, Path> {
        character::char('/').value(Path).parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

const PATH: Option<Operator> = Some(Operator::Level3(OpLevel3::Path(Path)));
const PATH_PREFIX: char = '/';
const PATH_INFIX: char = '/';

impl Expand<Values, Vec<VarSpec>> for Path {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        let mut defined = VarSpec::defined(context, value);

        if defined.peek().is_some() {
            output.push(PATH_PREFIX);
        }

        while let Some((value, var_spec)) = defined.next() {
            var_spec.expand(output, value, &PATH);

            if defined.peek().is_some() {
                output.push(PATH_INFIX);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Path {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.expand(output, value, &PATH),
            Value::List(value) => value
                .iter()
                .for_each(|value| codec::encode(value, output, common::reserved())),
            Value::AssociativeArray(value) => match context.1 {
                Some(Modifier::Explode(_)) => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push('=');
                    codec::encode(value, output, common::unreserved());
                }),
                _ => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push(PATH_INFIX);
                    codec::encode(value, output, common::unreserved());
                }),
            },
        }
    }
}

impl Expand<String, VarSpec> for Path {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        let len = value.len();
        let len = match context.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&value[..len], output, common::unreserved());
    }
}
