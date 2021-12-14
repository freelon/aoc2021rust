mod day11;
mod day12;
mod day13;
mod day14;
mod inputs;

use std::io::Write;
use std::time::Instant;

fn main() {
    let env = env_logger::Env::new().filter("RUST_LOG");
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .parse_env(env)
        .init();

    let input = inputs::input(2021, 14);

    let start = Instant::now();
    let result = day14::both_parts(&input, 40);
    println!("Computation took: {:?}", (Instant::now() - start));
    println!("Result: {:?}", result);
}
