use criterion::{criterion_group, criterion_main, Criterion};
use criterion_benchmark::{factorial, factorial_with_fold};

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.bench_function("Function 1", |b| b.iter(|| factorial(20)));
    group.bench_function("Function 2", |b| b.iter(|| factorial_with_fold(20)));
    group.finish();
}


criterion_group!(benches, benchmarks);
criterion_main!(benches);
