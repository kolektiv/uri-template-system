use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    BenchmarkId,
    Criterion,
};
use uri_template_system_fixtures::{
    self as fixtures,
    Group,
};

// =============================================================================
// Comparison
// =============================================================================

// Benchmarks

fn examples(c: &mut Criterion) {
    compare(c, "Examples", fixtures::examples());
}

fn examples_by_section(c: &mut Criterion) {
    compare(c, "Examples By Section", fixtures::examples_by_section());
}

fn extended_tests(c: &mut Criterion) {
    compare(c, "Extended Tests", fixtures::extended_tests());
}

// -----------------------------------------------------------------------------

// Comparison

fn compare(c: &mut Criterion, name: &str, groups: Vec<Group>) {
    let mut g = c.benchmark_group(name);

    for group in groups {
        let values = uri_template_system::prepare(group.variables.clone());

        g.bench_function(BenchmarkId::new(&group.name, "URI Template System"), |b| {
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

        g.bench_function(BenchmarkId::new(&group.name, "URITemplate Next"), |b| {
            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| uritemplate_next::test(&template, &group.variables)),
                    );
                },
                BatchSize::SmallInput,
            )
        });

        let context = iri_string::prepare(group.variables.clone());

        g.bench_function(BenchmarkId::new(&group.name, "IRI String"), |b| {
            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| iri_string::test(&template, &context)),
                    );
                },
                BatchSize::SmallInput,
            )
        });
    }

    g.finish();
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

// -----------------------------------------------------------------------------

// URITemplate Next

mod uritemplate_next {
    use uri_template_system_fixtures::Variable;
    use uritemplate::UriTemplate;

    pub fn test(template: &str, variables: &Vec<(String, Variable)>) -> String {
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

// -----------------------------------------------------------------------------

// IRI String

mod iri_string {
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
    use uri_template_system_fixtures::Variable;

    pub fn prepare(variables: Vec<(String, Variable)>) -> SimpleContext {
        variables
            .into_iter()
            .fold(SimpleContext::new(), |mut context, (n, v)| {
                match v {
                    Variable::AssociativeArray(v) => context.insert(n, Value::Assoc(v)),
                    Variable::Item(v) => context.insert(n, Value::String(v)),
                    Variable::List(v) => context.insert(n, Value::List(v)),
                };

                context
            })
    }

    pub fn test(template: &str, context: &SimpleContext) -> String {
        UriTemplateStr::new(template)
            .unwrap()
            .expand::<UriSpec, _>(context)
            .unwrap()
            .to_string()
    }
}

// =============================================================================
// Harness
// =============================================================================

criterion_group!(comparison, examples, examples_by_section, extended_tests);
criterion_main!(comparison);
