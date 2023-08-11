use crate::{
    codec::Encode,
    template::{
        common,
        expression::variable_specification::{
            self,
            VarSpec,
        },
        modifier::Modifier,
        variable_list::VariableList,
    },
    value::{
        Value,
        Values,
    },
    Expand,
    IndexMap,
};

// =============================================================================
// Simple
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct None;

// -----------------------------------------------------------------------------

// Expansion

const INFIX: char = ',';

impl<'a> Expand<Values, VariableList<'a>> for None {
    fn expand(&self, output: &mut String, values: &Values, variable_list: &VariableList<'a>) {
        let mut values = variable_specification::defined(variable_list, values);

        while let Some((value, var_spec)) = values.next() {
            self.expand(output, value, var_spec);

            if values.peek().is_some() {
                output.push(INFIX);
            }
        }
    }
}

impl<'a> Expand<Value, VarSpec<'a>> for None {
    fn expand(&self, output: &mut String, value: &Value, var_spec: &VarSpec) {
        match value {
            Value::Item(value) => self.expand(output, value, var_spec),
            Value::List(value) => self.expand(output, value, var_spec),
            Value::AssociativeArray(value) => self.expand(output, value, var_spec),
        }
    }
}

impl<'a> Expand<String, VarSpec<'a>> for None {
    fn expand(&self, output: &mut String, value: &String, var_spec: &VarSpec) {
        let len = value.len();
        let len = match &var_spec.1 {
            Some(Modifier::Prefix(prefix)) if len > prefix.length() => prefix.length(),
            _ => len,
        };

        output.push_str_encode(&value[..len], common::unreserved());
    }
}

impl<'a> Expand<Vec<String>, VarSpec<'a>> for None {
    fn expand(&self, output: &mut String, values: &Vec<String>, _var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        while let Some(value) = values.next() {
            output.push_str_encode(value, common::unreserved());

            if values.peek().is_some() {
                output.push(',');
            }
        }
    }
}

impl<'a> Expand<IndexMap<String, String>, VarSpec<'a>> for None {
    fn expand(&self, output: &mut String, values: &IndexMap<String, String>, var_spec: &VarSpec) {
        let mut values = values.iter().peekable();

        let infix = match var_spec.1 {
            Some(Modifier::Explode(_)) => '=',
            _ => ',',
        };

        while let Some((key, value)) = values.next() {
            output.push_str_encode(key, common::reserved());
            output.push(infix);
            output.push_str_encode(value, common::unreserved());

            if values.peek().is_some() {
                output.push(',');
            }
        }
    }
}
