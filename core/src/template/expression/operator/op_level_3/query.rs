use indexmap::IndexMap;
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
        expression::var_spec,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Query;

// -----------------------------------------------------------------------------

// Parsing

impl Query {
    pub fn parse(input: &str) -> IResult<&str, Query> {
        character::char('?').value(Query).parse(input)
    }
}

// -----------------------------------------------------------------------------

// Expansion

const PREFIX: char = '?';
const SEPARATOR: char = '&';

impl Expand<Values, Vec<VarSpec>> for Query {
    fn expand(&self, output: &mut String, values: &Values, var_specs: &Vec<VarSpec>) {
        let mut defined = var_spec::defined(var_specs, values);

        if defined.peek().is_some() {
            output.push(PREFIX);
        }

        while let Some((value, var_spec)) = defined.next() {
            self.expand(output, value, var_spec);

            if defined.peek().is_some() {
                output.push(SEPARATOR);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Query {
    fn expand(&self, output: &mut String, value: &Value, var_spec: &VarSpec) {
        match value {
            Value::Item(value) => self.expand(output, value, var_spec),
            Value::List(value) => self.expand(output, value, var_spec),
            Value::AssociativeArray(value) => self.expand(output, value, var_spec),
        }
    }
}

impl Expand<String, VarSpec> for Query {
    fn expand(&self, output: &mut String, value: &String, var_spec: &VarSpec) {
        let len = value.len();
        let len = match var_spec.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&var_spec.0, output, common::reserved());
        output.push('=');
        codec::encode(&value[..len], output, common::unreserved());
    }
}

impl Expand<Vec<String>, VarSpec> for Query {
    fn expand(&self, output: &mut String, values: &Vec<String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        match var_spec.1 {
            Some(Modifier::Explode(_)) => {
                while let Some(value) = values.next() {
                    codec::encode(&var_spec.0, output, common::reserved());
                    output.push('=');
                    codec::encode(value, output, common::unreserved());

                    if values.peek().is_some() {
                        output.push(SEPARATOR);
                    }
                }
            }
            _ => {
                codec::encode(&var_spec.0, output, common::reserved());
                output.push('=');

                while let Some(value) = values.next() {
                    codec::encode(value, output, common::unreserved());

                    if values.peek().is_some() {
                        output.push(',');
                    }
                }
            }
        }
    }
}

impl Expand<IndexMap<String, String>, VarSpec> for Query {
    fn expand(&self, output: &mut String, values: &IndexMap<String, String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        match var_spec.1 {
            Some(Modifier::Explode(_)) => {
                while let Some((key, value)) = values.next() {
                    codec::encode(key, output, common::reserved());
                    output.push('=');
                    codec::encode(value, output, common::unreserved());

                    if values.peek().is_some() {
                        output.push(SEPARATOR);
                    }
                }
            }
            _ => {
                codec::encode(&var_spec.0, output, common::reserved());
                output.push('=');

                while let Some((key, value)) = values.next() {
                    codec::encode(key, output, common::reserved());
                    output.push(',');
                    codec::encode(value, output, common::unreserved());

                    if values.peek().is_some() {
                        output.push(',');
                    }
                }
            }
        }
    }
}
