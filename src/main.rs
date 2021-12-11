mod day01;

use std::time::Instant;

fn main() {
    let input1 = "6636827465
6774248431
4227386366
7447452613
6223122545
2814388766
6615551144
4836235836
5334783256
4128344843
";

    let start = Instant::now();
    let result = day01::part1(input1);
    println!("Computation took: {:?}", (Instant::now() - start));
    println!("Result: {}", result);
}
