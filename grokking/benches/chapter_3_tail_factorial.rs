use criterion::{Criterion, criterion_group, criterion_main};
use grokking::chapter_3_tail_factorial::{factorial_with_tail, factorial_without_tail};

fn criterion_benchmark(c: &mut Criterion) {
    let number = 34;

    // c.bench_function("Factorial With Tail", |b| {
    //     b.iter(|| factorial_with_tail(number))
    // });

    c.bench_function("Factorial Without Tail", |b| {
        b.iter(|| factorial_without_tail(number))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
