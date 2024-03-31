#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

extern crate divisors_fixed;
use divisors_fixed::Divisors;

fn criterion_benchmark(c: &mut Criterion) {
    let n0: u64 = 12345678956u64;
    let n1: u64 = 93291493211u64;
    let n2: u64 = 11111111111u64;

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n0),
        move |b| b.iter(|| black_box(n0).divisors()),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n0),
        move |b| b.iter(|| get_divisors_standard(black_box(n0))),
    );

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n1),
        move |b| b.iter(|| black_box(n1).divisors()),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n1),
        move |b| b.iter(|| get_divisors_standard(black_box(n1))),
    );

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n2),
        move |b| b.iter(|| black_box(n2).divisors()),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n2),
        move |b| b.iter(|| get_divisors_standard(black_box(n2))),
    );
}

/// geeksforgeeks solution: https://www.geeksforgeeks.org/find-divisors-natural-number-set-1/
fn get_divisors_standard(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let n_sqrt = (n as f64).sqrt() as u64 + 1;

    for i in 1..n_sqrt {
        if n % i == 0 {
            if n / i == i {
                v.push(i);
            } else {
                v.push(i);
                v.push(n / i);
            }
        }
    }
    v.sort();
    v
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
