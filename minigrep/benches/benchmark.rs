use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minigrep::{search, search_case_insensitive};

fn criterion_benchmark(c: &mut Criterion) {
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody! Nobody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    c.bench_function("search", |b| {
        b.iter(|| search(black_box("Nobody"), black_box(contents)));
        b.iter(|| search(black_box("nobody"), black_box(contents)))
    });

    c.bench_function("search_case_insensitive", |b| {
        b.iter(|| search_case_insensitive(black_box("Nobody"), black_box(contents)));
        b.iter(|| search_case_insensitive(black_box("nobody"), black_box(contents)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
