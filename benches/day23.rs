use aoc2021rust::day23::*;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

const INPUT2: &str = "#############
#...........#
###B#B#D#D###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#A#C#
  #########
";

pub fn day23_bench(c: &mut Criterion) {
    c.bench_function("day 23 part 2", |b| b.iter(|| part1(INPUT)));
}

pub fn day23_2_bench(c: &mut Criterion) {
    c.bench_function("day 23 part 2", |b| b.iter(|| part1(INPUT2)));
}

use std::time::Duration;
criterion_group!(name = fast;
    config = Criterion::default()
    .warm_up_time(Duration::from_secs(10))
    .sample_size(15)
    .measurement_time(Duration::from_secs(50));
    targets = day23_bench);
criterion_group!(name = slow;
    config = Criterion::default()
    .warm_up_time(Duration::from_secs(30))
    .sample_size(10)
    .measurement_time(Duration::from_secs(300));
    targets =  day23_2_bench);
criterion_main!(fast, slow);
