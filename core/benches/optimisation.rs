use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use uri_template_system_core::fibonacci;

// =============================================================================
// Optimisation
// =============================================================================

static FIXTURES_DATA: &str = "../fixtures/data";

pub fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// Groups

criterion_group!(optimisation, fibonacci_benchmark);

// Main

criterion_main!(optimisation);
