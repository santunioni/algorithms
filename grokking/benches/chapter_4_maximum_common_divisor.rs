use criterion::{Criterion, criterion_group, criterion_main};
use grokking::chapter_4_maximum_common_divisor::{
    greatest_common_divisor_recursive_euclides, greatest_common_divisor_simpler_but_slow,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Euclid's Algorithm for GCD", |b| {
        b.iter(|| greatest_common_divisor_recursive_euclides(1680, 640))
    });

    c.bench_function("Simple Algorithm for GCD", |b| {
        b.iter(|| greatest_common_divisor_simpler_but_slow(1680, 640))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
