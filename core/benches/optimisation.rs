use std::path::PathBuf;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use uri_template_system_core::{
    URITemplate,
    Value,
    Values,
};
use uri_template_system_fixtures::{
    Case,
    Expansion,
    Group,
    Variable,
};

// =============================================================================
// Optimisation
// =============================================================================

static FIXTURES_DATA: &str = "../fixtures/data";

fn spec_examples(criterion: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples.json");
    let groups = uri_template_system_fixtures::load(path);

    benchmark(criterion, groups);
}

fn spec_examples_by_section(criterion: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples-by-section.json");
    let groups = uri_template_system_fixtures::load(path);

    benchmark(criterion, groups);
}

fn extended_tests(criterion: &mut Criterion) {
    let path = PathBuf::from(FIXTURES_DATA).join("extended-tests.json");
    let groups = uri_template_system_fixtures::load(path);

    benchmark(criterion, groups);
}

fn benchmark(criterion: &mut Criterion, groups: Vec<Group>) {
    for Group {
        name,
        variables,
        cases,
    } in groups
    {
        let values = Values::from_iter(
            variables
                .into_iter()
                .filter_map(|(name, variable)| match variable {
                    Variable::AssociativeArray(value) => {
                        Some((name, Value::AssociativeArray(value)))
                    }
                    Variable::Item(value) => Some((name, Value::Item(value))),
                    Variable::List(value) => Some((name, Value::List(value))),
                    Variable::Number(value) => Some((name, Value::Item(value.to_string()))),
                    Variable::Undefined => None,
                })
                .collect::<Vec<_>>(),
        );

        criterion.bench_function(&name, |bencher| {
            bencher.iter(|| {
                for Case {
                    expansion,
                    template,
                } in &cases
                {
                    let actual = URITemplate::parse(black_box(template))
                        .unwrap()
                        .expand(black_box(&values));

                    match expansion {
                        Expansion::String(expected) => assert!(expected == &actual),
                        Expansion::List(expected) => assert!(expected.contains(&actual)),
                    };
                }
            })
        });
    }
}

// Groups

criterion_group!(
    optimisation,
    spec_examples,
    spec_examples_by_section,
    extended_tests
);

// Main

criterion_main!(optimisation);
