use std::path::PathBuf;

use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};
use uri_template_system_fixtures::{
    Expansion,
    Group,
};

// =============================================================================
// Comparison
// =============================================================================

// Benchmarks

static FIXTURES_DATA: &str = "../fixtures/data";

fn spec_examples(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples.json");
    let groups = uri_template_system_fixtures::load(path);

    compare(c, "1. Spec Examples", groups);
}

fn spec_examples_by_section(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples-by-section.json");
    let groups = uri_template_system_fixtures::load(path);

    compare(c, "2. Spec Examples By Section", groups);
}

fn extended_tests(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("extended-tests.json");
    let groups = uri_template_system_fixtures::load(path);

    compare(c, "3. Extended Tests", groups);
}

// -----------------------------------------------------------------------------

// Comparison

fn compare(c: &mut Criterion, name: &str, groups: Vec<Group>) {
    let mut g = c.benchmark_group(name);

    for group in groups {
        g.bench_function(BenchmarkId::new("URI Template System", &group.name), |b| {
            b.iter(|| {
                for case in &group.cases {
                    let template = &case.template;
                    let variables = group.variables.clone();
                    let actual = uri_template_system::expand(template, variables);

                    match &case.expansion {
                        Expansion::Single(expected) => assert!(expected == &actual),
                        Expansion::Multiple(expected) => assert!(expected.contains(&actual)),
                    };
                }
            })
        });

        g.bench_function(BenchmarkId::new("URITemplate Next", &group.name), |b| {
            b.iter(|| {
                for case in &group.cases {
                    let template = &case.template;
                    let variables = group.variables.clone();
                    let actual = uritemplate_next::expand(template, variables);

                    match &case.expansion {
                        Expansion::Single(expected) => assert!(expected == &actual),
                        Expansion::Multiple(expected) => assert!(expected.contains(&actual)),
                    };
                }
            })
        });
    }

    g.finish();
}

mod uri_template_system {
    use indexmap::IndexMap;
    use uri_template_system_core::{
        URITemplate,
        Value,
        Values,
    };
    use uri_template_system_fixtures::Variable;

    pub fn expand(template: &str, variables: Vec<(String, Variable)>) -> String {
        URITemplate::parse(template)
            .unwrap()
            .expand(&to_values(variables))
    }

    fn to_values(variables: Vec<(String, Variable)>) -> Values {
        Values::from_iter(variables.into_iter().map(to_value))
    }

    fn to_value((n, v): (String, Variable)) -> (String, Value) {
        match v {
            Variable::AssociativeArray(v) => (n, Value::AssociativeArray(IndexMap::from_iter(v))),
            Variable::Item(v) => (n, Value::Item(v)),
            Variable::List(v) => (n, Value::List(v)),
        }
    }
}

mod uritemplate_next {
    use uri_template_system_fixtures::Variable;
    use uritemplate::UriTemplate;

    pub fn expand(template: &str, variables: Vec<(String, Variable)>) -> String {
        let mut template = UriTemplate::new(template);

        variables.into_iter().for_each(|(n, v)| {
            match v {
                Variable::AssociativeArray(v) => template.set(&n, v),
                Variable::Item(v) => template.set(&n, v),
                Variable::List(v) => template.set(&n, v),
            };
        });

        template.build()
    }
}

// -----------------------------------------------------------------------------

// Groups

criterion_group!(
    comparison,
    spec_examples,
    spec_examples_by_section,
    extended_tests
);

// -----------------------------------------------------------------------------

// Main

criterion_main!(comparison);
