use uritemplate::UriTemplate;

use crate::{
    fixtures::Variable,
    harnesses,
};

pub struct Harness;

impl harnesses::Harness for Harness {
    type Values = Vec<(String, Variable)>;

    fn prepare(&self, variables: Vec<(String, Variable)>) -> Self::Values {
        variables
    }

    fn test(&self, template: &str, variables: &Vec<(String, Variable)>) -> String {
        variables
            .into_iter()
            .fold(UriTemplate::new(template), |mut template, (n, v)| {
                match v {
                    Variable::AssociativeArray(v) => template.set(&n, &v[..]),
                    Variable::Item(v) => template.set(n, v.as_str()),
                    Variable::List(v) => template.set(n, &v[..]),
                };

                template
            })
            .build()
    }
}
