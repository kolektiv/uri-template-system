use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    BenchmarkId,
    Criterion,
};
use uri_template_system_tests::{
    fixtures::{
        self,
        Case,
        Group,
    },
    harnesses::{
        iri_string,
        uri_template_next,
        uri_template_system,
        Harness,
    },
};

// =============================================================================
// Comparison
// =============================================================================

// Benchmarks

fn bench_sets(c: &mut Criterion) {
    bench_set(c, "Examples", fixtures::examples());
    bench_set(c, "Examples By Section", fixtures::examples_by_section());
    bench_set(c, "Extended Tests", fixtures::extended_tests());
}

fn bench_set(c: &mut Criterion, name: &str, groups: Vec<Group>) {
    let mut g = c.benchmark_group(name);

    for group in groups {
        g.bench_function(BenchmarkId::new(&group.name, "URI Template System"), |b| {
            let harness = uri_template_system::Harness;
            let values = harness.prepare(group.variables.clone());

            b.iter_batched_ref(
                || setup(&group.cases),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| harness.test(&template, &values)),
                    );
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(BenchmarkId::new(&group.name, "URITemplate Next"), |b| {
            let harness = uri_template_next::Harness;

            b.iter_batched_ref(
                || setup(&group.cases),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| harness.test(&template, &group.variables)),
                    );
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(BenchmarkId::new(&group.name, "IRI String"), |b| {
            let harness = iri_string::Harness;
            let context = harness.prepare(group.variables.clone());

            b.iter_batched_ref(
                || setup(&group.cases),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input
                            .iter()
                            .map(|template| harness.test(&template, &context)),
                    );
                },
                BatchSize::SmallInput,
            )
        });
    }

    g.finish();
}

fn setup(cases: &Vec<Case>) -> (Vec<String>, Vec<String>) {
    (
        cases.iter().map(|c| c.template.clone()).collect(),
        Vec::with_capacity(cases.len()),
    )
}

// =============================================================================
// Harness
// =============================================================================

criterion_group!(comparison, bench_sets);
criterion_main!(comparison);
