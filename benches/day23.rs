use aoc2021rust::day23::*;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

pub fn day23_bench(c: &mut Criterion) {
    c.bench_function("day 23 part 1", |b| b.iter(|| part1(INPUT)));
}

use std::time::Duration;
criterion_group!(name = bencher;
    config = Criterion::default()
    .warm_up_time(Duration::from_secs(10))
    .sample_size(15)
    .measurement_time(Duration::from_secs(50));
    targets = day23_bench);
criterion_main!(bencher);
