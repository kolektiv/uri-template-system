use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    Criterion,
};
use uri_template_system_tests::{
    fixtures::{
        self,
        Group,
    },
    harnesses::{
        uri_template_system,
        Harness,
    },
};

// =============================================================================
// Optimisation
// =============================================================================

// Benchmarks

fn bench_sets(c: &mut Criterion) {
    bench_set(c, "Examples", fixtures::examples());
    bench_set(c, "Examples By Section", fixtures::examples_by_section());
    bench_set(c, "Extended Tests", fixtures::extended_tests());
}

fn bench_set(c: &mut Criterion, name: &str, groups: Vec<Group>) {
    for group in groups {
        let harness = uri_template_system::Harness;
        let values = harness.prepare(group.variables.clone());

        c.bench_function(&format!("{}: {}", name, &group.name), |b| {
            b.iter_batched_ref(
                || setup(&group),
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
    }
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

criterion_group!(optimisation, bench_sets);
criterion_main!(optimisation);
