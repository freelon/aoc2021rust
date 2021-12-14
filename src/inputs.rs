use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use reqwest::blocking::Client;

pub fn input(year: u16, day: u8) -> String {
    let mut input = String::new();
    let path = format!("input/{}/day{}.txt", year, day);
    let path = Path::new(&path);
    if let Ok(mut file) = File::open(&path) {
        file.read_to_string(&mut input)
            .expect("failed to read file contents");
    } else {
        input = load_external(year, day);

        let mut file = File::create(&path).unwrap();
        write!(file, "{}", input).unwrap();
    }
    input
}

fn load_external(year: u16, day: u8) -> String {
    let session_id = include!("../session_secret");

    let client = Client::new();
    let res = client.get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .header("Cookie", format!("session={}", session_id))
    .send()
    .expect("Couldn't get input from server");
    if !res.status().is_success() {
        panic!("Bad response: {}", res.status());
    }

    res.text().expect("couldn't read body")
}
