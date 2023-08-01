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
pub struct Label;

// -----------------------------------------------------------------------------

// Parsing

impl Label {
    pub fn parse(input: &str) -> IResult<&str, Label> {
        character::char('.').value(Label).parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

const LABEL: Option<Operator> = Some(Operator::Level3(OpLevel3::Label(Label)));
const LABEL_PREFIX: char = '.';
const LABEL_INFIX: char = '.';

impl Expand<Values, Vec<VarSpec>> for Label {
    fn expand(&self, output: &mut String, value: &Values, context: &Vec<VarSpec>) {
        let mut defined = VarSpec::defined(context, value);

        if defined.peek().is_some() {
            output.push(LABEL_PREFIX);
        }

        while let Some((value, var_spec)) = defined.next() {
            var_spec.expand(output, value, &LABEL);

            if defined.peek().is_some() {
                output.push(LABEL_INFIX);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Label {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.expand(output, value, &LABEL),
            Value::List(value) => value
                .iter()
                .for_each(|value| codec::encode(value, output, common::unreserved())),
            Value::AssociativeArray(value) => match context.1 {
                Some(Modifier::Explode(_)) => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push('=');
                    codec::encode(value, output, common::unreserved());
                }),
                _ => value.iter().for_each(|(key, value)| {
                    output.push_str(key);
                    output.push(LABEL_INFIX);
                    codec::encode(value, output, common::unreserved());
                }),
            },
        }
    }
}

impl Expand<String, VarSpec> for Label {
    fn expand(&self, output: &mut String, value: &String, context: &VarSpec) {
        let len = value.len();
        let len = match context.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&value[..len], output, common::unreserved());
    }
}
