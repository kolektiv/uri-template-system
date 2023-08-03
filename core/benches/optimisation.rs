use std::path::PathBuf;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use indexmap::IndexMap;
use uri_template_system_core::{
    URITemplate,
    Value,
    Values,
};
use uri_template_system_fixtures::{
    Expansion,
    Group,
    Variable,
};

// =============================================================================
// Optimisation
// =============================================================================

// Benchmarks

static FIXTURES_DATA: &str = "../fixtures/data";

fn spec_examples(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples.json");
    let groups = uri_template_system_fixtures::load(path);

    measure(c, groups);
}

fn spec_examples_by_section(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples-by-section.json");
    let groups = uri_template_system_fixtures::load(path);

    measure(c, groups);
}

fn extended_tests(c: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("extended-tests.json");
    let groups = uri_template_system_fixtures::load(path);

    measure(c, groups);
}

// -----------------------------------------------------------------------------

// Measurement

fn measure(c: &mut Criterion, groups: Vec<Group>) {
    for group in groups {
        let values = to_values(group.variables);

        c.bench_function(&group.name, |b| {
            b.iter(|| {
                for case in &group.cases {
                    let actual = URITemplate::parse(black_box(&case.template))
                        .unwrap()
                        .expand(black_box(&values));

                    match &case.expansion {
                        Expansion::Single(expected) => assert!(expected == &actual),
                        Expansion::Multiple(expected) => assert!(expected.contains(&actual)),
                    };
                }
            })
        });
    }
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

// -----------------------------------------------------------------------------

// Groups

criterion_group!(
    optimisation,
    spec_examples,
    spec_examples_by_section,
    extended_tests
);

// -----------------------------------------------------------------------------

// Main

criterion_main!(optimisation);
