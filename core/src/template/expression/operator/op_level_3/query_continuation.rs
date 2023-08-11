use crate::{
    codec::Encode,
    template::{
        common,
        expression::{
            modifier::Modifier,
            variable_list::VariableList,
            variable_specification::{
                self,
                VarSpec,
            },
        },
    },
    value::{
        Value,
        Values,
    },
    Expand,
    IndexMap,
};

#[derive(Debug, Eq, PartialEq)]
pub struct QueryContinuation<'a> {
    raw: &'a str,
}

impl<'a> QueryContinuation<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expansion

const PREFIX: char = '&';
const SEPARATOR: char = '&';

impl<'a> Expand<Values, VariableList<'a>> for QueryContinuation<'a> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        let mut defined = variable_specification::defined(variable_list, values);

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

impl<'a> Expand<Value, VarSpec<'a>> for QueryContinuation<'a> {
    fn expand(&self, output: &mut String, value: &Value, var_spec: &VarSpec) {
        match value {
            Value::Item(value) => self.expand(output, value, var_spec),
            Value::List(value) => self.expand(output, value, var_spec),
            Value::AssociativeArray(value) => self.expand(output, value, var_spec),
        }
    }
}

impl<'a> Expand<String, VarSpec<'a>> for QueryContinuation<'a> {
    fn expand(&self, output: &mut String, value: &String, var_spec: &VarSpec) {
        let len = value.len();
        let len = match &var_spec.1 {
            Some(Modifier::Prefix(prefix)) if len > prefix.length() => prefix.length(),
            _ => len,
        };

        output.push_str_encode(var_spec.0.value(), common::reserved());
        output.push('=');
        output.push_str_encode(&value[..len], common::unreserved());
    }
}

impl<'a> Expand<Vec<String>, VarSpec<'a>> for QueryContinuation<'a> {
    fn expand(&self, output: &mut String, values: &Vec<String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        match var_spec.1 {
            Some(Modifier::Explode(_)) => {
                while let Some(value) = values.next() {
                    output.push_str_encode(var_spec.0.value(), common::reserved());
                    output.push('=');
                    output.push_str_encode(value, common::unreserved());

                    if values.peek().is_some() {
                        output.push(SEPARATOR);
                    }
                }
            }
            _ => {
                output.push_str_encode(var_spec.0.value(), common::reserved());
                output.push('=');

                while let Some(value) = values.next() {
                    output.push_str_encode(value, common::unreserved());

                    if values.peek().is_some() {
                        output.push(',');
                    }
                }
            }
        }
    }
}

impl<'a> Expand<IndexMap<String, String>, VarSpec<'a>> for QueryContinuation<'a> {
    fn expand(&self, output: &mut String, values: &IndexMap<String, String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        match var_spec.1 {
            Some(Modifier::Explode(_)) => {
                while let Some((key, value)) = values.next() {
                    output.push_str_encode(key, common::reserved());
                    output.push('=');
                    output.push_str_encode(value, common::unreserved());

                    if values.peek().is_some() {
                        output.push(SEPARATOR);
                    }
                }
            }
            _ => {
                output.push_str_encode(var_spec.0.value(), common::reserved());
                output.push('=');

                while let Some((key, value)) = values.next() {
                    output.push_str_encode(key, common::reserved());
                    output.push(',');
                    output.push_str_encode(value, common::unreserved());

                    if values.peek().is_some() {
                        output.push(',');
                    }
                }
            }
        }
    }
}
