use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    Criterion,
};
use uri_template_system_fixtures::{
    self as fixtures,
    Group,
};

// =============================================================================
// Optimisation
// =============================================================================

// Benchmarks

fn examples(c: &mut Criterion) {
    measure(c, "Examples", fixtures::examples());
}

fn examples_by_section(c: &mut Criterion) {
    measure(c, "Examples By Section", fixtures::examples_by_section());
}

fn extended_tests(c: &mut Criterion) {
    measure(c, "Extended Tests", fixtures::extended_tests());
}

// -----------------------------------------------------------------------------

// Measurement

fn measure(c: &mut Criterion, name: &str, groups: Vec<Group>) {
    for group in groups {
        let values = uri_template_system::prepare(group.variables.clone());

        c.bench_function(&format!("{}: {}", name, &group.name), |b| {
            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| uri_template_system::test(&template, &values)),
                    );
                },
                BatchSize::SmallInput,
            )
        });
    }
}

fn setup(group: &Group) -> (Vec<String>, Vec<String>) {
    (
        group.cases.iter().map(|c| c.template.clone()).collect(),
        Vec::with_capacity(group.cases.len()),
    )
}

// =============================================================================
// Implementations
// =============================================================================

// URI Template System

mod uri_template_system {
    use uri_template_system_core::{
        URITemplate,
        Value,
        Values,
    };
    use uri_template_system_fixtures::Variable;

    pub fn prepare(variables: Vec<(String, Variable)>) -> Values {
        Values::from_iter(variables.into_iter().map(|(n, v)| match v {
            Variable::AssociativeArray(v) => (n, Value::AssociativeArray(v)),
            Variable::Item(v) => (n, Value::Item(v)),
            Variable::List(v) => (n, Value::List(v)),
        }))
    }

    pub fn test(template: &str, values: &Values) -> String {
        URITemplate::parse(template)
            .unwrap()
            .expand(values)
            .to_string()
    }
}

// =============================================================================
// Harness
// =============================================================================

criterion_group!(optimisation, examples, examples_by_section, extended_tests);
criterion_main!(optimisation);
