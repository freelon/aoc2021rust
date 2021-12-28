use aoc2021rust::day19::*;
use criterion::{criterion_group, Criterion};

use bencher::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 19", |b| {
        b.iter(|| part1(include!("../src/day19_test_data.rs")))
    });
}

use std::time::Duration;
criterion_group!(name = benches;
    config = Criterion::default()
    .warm_up_time(Duration::from_secs(20))
    .measurement_time(Duration::from_secs(120));
    targets = criterion_benchmark);
// criterion_main!(benches);

fn blubb(b: &mut Bencher) {
    b.iter(||part1(include!("../src/day19_test_data.rs")));
}

benchmark_group!(bencher_group, blubb);
benchmark_main!(bencher_group);