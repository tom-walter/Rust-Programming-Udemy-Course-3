// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exercise_f::sploosh;

pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function(
        "sploosh 2", |b| b.iter(
            || sploosh(black_box(8), black_box(9), black_box(10))
        )
    );
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);