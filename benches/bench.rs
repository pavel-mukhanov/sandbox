#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use std::collections::HashMap;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_opt(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    let cached = *cache.get(&n).unwrap_or(&0);

    if cached > 0 {
        return cached;
    }

    let res = match n {
        0 => 1,
        1 => 1,
        _ => fibonacci_opt(n - 1, cache) + fibonacci_opt(n - 2, cache),
    };
    cache.insert(n, res);
    res
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 50", |b| b.iter(|| fibonacci(black_box(50))));

    c.bench_function("fib_opt 50", |b| {
        b.iter(|| fibonacci_opt(black_box(50), &mut HashMap::new()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
