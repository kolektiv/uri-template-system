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
        Group,
    },
    harnesses::{
        self,
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
            let values = harnesses::uri_template_system::Harness.prepare(group.variables.clone());

            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(input.iter().map(|template| {
                        harnesses::uri_template_system::Harness.test(&template, &values)
                    }));
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(BenchmarkId::new(&group.name, "URITemplate Next"), |b| {
            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(input.iter().map(|template| {
                        harnesses::uri_template_next::Harness.test(&template, &group.variables)
                    }));
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(BenchmarkId::new(&group.name, "IRI String"), |b| {
            let context = harnesses::iri_string::Harness.prepare(group.variables.clone());

            b.iter_batched_ref(
                || setup(&group),
                |(input, output): &mut (Vec<String>, Vec<String>)| {
                    output.extend(
                        input.iter().map(|template| {
                            harnesses::iri_string::Harness.test(&template, &context)
                        }),
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
// Harness
// =============================================================================

criterion_group!(comparison, bench_sets);
criterion_main!(comparison);
