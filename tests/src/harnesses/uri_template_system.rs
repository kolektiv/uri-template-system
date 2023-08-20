use uri_template_system_core::{
    Template,
    Value,
    Values,
};

use crate::fixtures::Variable;

pub struct Harness;

impl super::Harness for Harness {
    type Values = Values;

    fn prepare(&self, variables: Vec<(String, Variable)>) -> Self::Values {
        variables
            .into_iter()
            .fold(Values::default(), |values, (n, v)| match v {
                Variable::AssociativeArray(v) => values.add(n, Value::AssociativeArray(v)),
                Variable::Item(v) => values.add(n, Value::Item(v)),
                Variable::List(v) => values.add(n, Value::List(v)),
            })
    }

    fn test(&self, template: &str, values: &Self::Values) -> String {
        Template::parse(template).unwrap().expand(values).unwrap()
    }
}
