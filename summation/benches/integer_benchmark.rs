#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

fn sum_formula(n: u128) -> u128 {
    let sum = (n / 2) * (n - 1);

    sum
}

fn sum_loop(n: u128) -> u128 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }

    sum
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("thousand loop benchmark", |b| b.iter(|| sum_loop(black_box(1000))));
    c.bench_function("thousand formula benchmark", |b| b.iter(|| sum_formula(black_box(1000))));

    c.bench_function("billion loop benchmark", |b| b.iter(|| sum_loop(black_box(1_000_000_000))));
    c.bench_function("billion formula benchmark", |b| b.iter(|| sum_formula(black_box(1_000_000_000))));

    // Quintillion loop not tested as it takes too long
    // c.bench_function("quintillion loop benchmark", |b| b.iter(|| sum_loop(black_box(1_000_000_000_000_000))));
    c.bench_function("quintillion formula benchmark", |b| b.iter(|| sum_formula(black_box(1_000_000_000_000_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);