use std::collections::HashMap;

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

const INFIX: char = ',';

impl Expand<Values, Vec<VarSpec>> for Reserved {
    fn expand(&self, output: &mut String, values: &Values, var_specs: &Vec<VarSpec>) {
        let mut values = var_spec::defined(var_specs, values);

        while let Some((value, var_spec)) = values.next() {
            self.expand(output, value, var_spec);

            if values.peek().is_some() {
                output.push(INFIX);
            }
        }
    }
}

impl Expand<Value, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, value: &Value, var_spec: &VarSpec) {
        match value {
            Value::Item(value) => self.expand(output, value, var_spec),
            Value::List(value) => self.expand(output, value, var_spec),
            Value::AssociativeArray(value) => self.expand(output, value, var_spec),
        }
    }
}

impl Expand<String, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, value: &String, var_spec: &VarSpec) {
        let len = value.len();
        let len = match var_spec.1 {
            Some(Modifier::Prefix(Prefix(max_len))) if len > max_len => max_len,
            _ => len,
        };

        codec::encode(&value[..len], output, common::reserved());
    }
}

impl Expand<Vec<String>, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, values: &Vec<String>, _var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        while let Some(value) = values.next() {
            codec::encode(value, output, common::reserved());

            if values.peek().is_some() {
                output.push(INFIX);
            }
        }
    }
}

impl Expand<HashMap<String, String>, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, values: &HashMap<String, String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        let infix = match var_spec.1 {
            Some(Modifier::Explode(_)) => '=',
            _ => ',',
        };

        while let Some((key, value)) = values.next() {
            codec::encode(key, output, common::reserved());
            output.push(infix);
            codec::encode(value, output, common::reserved());

            if values.peek().is_some() {
                output.push(',');
            }
        }
    }
}
