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
    ParseRef,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Label<'a> {
    parse_ref: ParseRef<'a>,
}

impl<'a> Label<'a> {
    pub fn new(parse_ref: ParseRef<'a>) -> Self {
        Self { parse_ref }
    }
}

// -----------------------------------------------------------------------------

// Expansion

const PREFIX: char = '.';
const SEPARATOR: char = '.';

impl<'a> Expand<Values, VariableList<'a>> for Label<'a> {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        let mut values = variable_specification::defined(variable_list, values);

        if values.peek().is_some() {
            output.push(PREFIX);
        }

        while let Some((value, var_spec)) = values.next() {
            self.expand(output, value, var_spec);

            if values.peek().is_some() {
                output.push(SEPARATOR);
            }
        }
    }
}

impl<'a> Expand<Value, VarSpec<'a>> for Label<'a> {
    fn expand(&self, output: &mut String, value: &Value, var_spec: &VarSpec) {
        match value {
            Value::Item(value) => self.expand(output, value, var_spec),
            Value::List(value) => self.expand(output, value, var_spec),
            Value::AssociativeArray(value) => self.expand(output, value, var_spec),
        }
    }
}

impl<'a> Expand<String, VarSpec<'a>> for Label<'a> {
    fn expand(&self, output: &mut String, value: &String, var_spec: &VarSpec) {
        let len = value.len();
        let len = match &var_spec.1 {
            Some(Modifier::Prefix(prefix)) if len > prefix.length() => prefix.length(),
            _ => len,
        };

        output.push_str_encode(&value[..len], common::unreserved());
    }
}

impl<'a> Expand<Vec<String>, VarSpec<'a>> for Label<'a> {
    fn expand(&self, output: &mut String, values: &Vec<String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        let separator = match var_spec.1 {
            Some(Modifier::Explode(_)) => SEPARATOR,
            _ => ',',
        };

        while let Some(value) = values.next() {
            output.push_str_encode(&value, common::reserved());

            if values.peek().is_some() {
                output.push(separator);
            }
        }
    }
}

impl<'a> Expand<IndexMap<String, String>, VarSpec<'a>> for Label<'a> {
    fn expand(&self, output: &mut String, values: &IndexMap<String, String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        let (infix, separator) = match var_spec.1 {
            Some(Modifier::Explode(_)) => ('=', SEPARATOR),
            _ => (',', ','),
        };

        while let Some((key, value)) = values.next() {
            output.push_str_encode(key, common::reserved());
            output.push(infix);
            output.push_str_encode(value, common::unreserved());

            if values.peek().is_some() {
                output.push(separator);
            }
        }
    }
}
