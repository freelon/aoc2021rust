use aoc2021rust::day19::*;
use bencher::*;

fn blubb(b: &mut Bencher) {
    b.iter(|| part1(include!("../src/day19_test_data.rs")));
}

benchmark_group!(bencher_group, blubb);
benchmark_main!(bencher_group);
