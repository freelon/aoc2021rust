mod day01;

use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

fn main() {
    let env = env_logger::Env::new().filter("RUST_LOG");
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .parse_env(env)
        .init();

    let mut input = String::new();
    File::open(&Path::new("input/day01.txt"))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let start = Instant::now();
    let result = day01::part2(&input);
    println!("Computation took: {:?}", (Instant::now() - start));
    println!("Result: {}", result);
}
