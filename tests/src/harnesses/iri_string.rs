use iri_string::{
    spec::UriSpec,
    template::{
        simple_context::{
            SimpleContext,
            Value,
        },
        UriTemplateStr,
    },
};

use crate::{
    fixtures::Variable,
    harnesses,
};

pub struct Harness;

impl harnesses::Harness for Harness {
    type Values = SimpleContext;

    fn prepare(&self, variables: Vec<(String, Variable)>) -> Self::Values {
        variables
            .into_iter()
            .fold(SimpleContext::new(), |mut context, (n, v)| {
                match v {
                    Variable::AssociativeArray(v) => context.insert(n, Value::Assoc(v)),
                    Variable::Item(v) => context.insert(n, Value::String(v)),
                    Variable::List(v) => context.insert(n, Value::List(v)),
                    Variable::Undefined => context.insert(n, Value::Undefined),
                };

                context
            })
    }

    fn test(&self, template: &str, context: &SimpleContext) -> String {
        UriTemplateStr::new(template)
            .unwrap()
            .expand::<UriSpec, _>(context)
            .unwrap()
            .to_string()
    }
}
