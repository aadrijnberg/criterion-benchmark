use criterion::{criterion_group, criterion_main, Criterion};
use criterion_benchmark::factorial;

fn benchmarks(c: &mut Criterion) {
    c.bench_function("factorial 20", |b| b.iter(|| factorial(20)));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
