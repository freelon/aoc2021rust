use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use reqwest::blocking::Client;

pub fn input(year: u16, day: u8) -> String {
    let mut input = String::new();
    if let Ok(mut file) = File::open(&Path::new(&format!("input/{}/day{}.txt", year, day))) {
        file.read_to_string(&mut input)
            .expect("failed to read file contents");
    } else {
        input = load_external(year, day);

        // TODO store the retrieves contents
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
