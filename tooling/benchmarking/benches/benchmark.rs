use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add two numbers", |b| b.iter(|| add(black_box(2), black_box(3))));
}

criterion_group!(benches, benchmark_add);
criterion_main!(benches);
