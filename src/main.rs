mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod inputs;

use std::io::Write;
use std::time::Instant;

fn main() {
    let env = env_logger::Env::new().filter("RUST_LOG");
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .parse_env(env)
        .init();

    let input = inputs::input(2021, 16);

    let start = Instant::now();
    let result = day16::part2(&input);
    println!("Computation took: {:?}", start.elapsed());
    println!("Result: {:?}", result);
}
