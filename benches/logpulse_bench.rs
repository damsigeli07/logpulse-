use criterion::{Criterion, criterion_group, criterion_main};

use logpulse::pipeline::{parse_logs_parallel, parse_logs_sequential};
use std::hint::black_box;

fn benchmark_content() -> String {
    let line = include_str!("../samples/access.log")
        .lines()
        .next()
        .expect("sample log should contain a line");

    std::iter::repeat(line)
        .take(100_000)
        .collect::<Vec<_>>()
        .join("\n")
}

fn benchmark_sequential(c: &mut Criterion) {
    let content = benchmark_content();

    c.bench_function("sequential_log_parsing", |b| {
        b.iter(|| parse_logs_sequential(black_box(content)));
    });
}

fn benchmark_parallel(c: &mut Criterion) {
    let content = benchmark_content();

    c.bench_function("parallel_log_parsing", |b| {
        b.iter(|| parse_logs_parallel(black_box(content)));
    });
}

criterion_group!(benches, benchmark_sequential, benchmark_parallel);

criterion_main!(benches);
