use criterion::{criterion_group, criterion_main, Criterion};
use minigrep_santunioni::run_with_command;
use std::fs;

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

    let file_path = "/tmp/benchmark-text.txt";

    fs::write(&file_path, contents).expect("Benchmark text should be writtem");

    let command_case_sensitive = format!("minigrep --file-path={file_path} --query=nobody");
    let command_case_insensitive =
        format!("minigrep --file-path={file_path} --query=Nobody --ignore-case");

    c.bench_function("search case sensitive", |b| {
        b.iter(|| run_with_command(&command_case_sensitive, false))
    });

    c.bench_function("search case insensitive", |b| {
        b.iter(|| run_with_command(&command_case_insensitive, false))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
