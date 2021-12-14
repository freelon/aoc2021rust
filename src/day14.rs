#![allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (template, rules) = input.trim().split_once("\n\n").unwrap();
    let rules: HashMap<&str, &str> = rules
        .split("\n")
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();

    let mut polymer: String = template.to_string();
    for _i in 0..10 {
        process(&mut polymer, &rules);
    }
    let frequencies = polymer.chars().counts();
    let hfreq = frequencies.iter().max_by_key(|(_k, v)| *v).unwrap();
    let lfreq = frequencies.iter().min_by_key(|(_k, v)| *v).unwrap();

    hfreq.1 - lfreq.1
}

fn process(poly: &mut String, rules: &HashMap<&str, &str>) {
    let mut i = 0;
    while i < poly.len() - 1 {
        let part = &poly[i..=i + 1];
        if let Some(c) = rules.get(part) {
            poly.insert(i + 1, c.chars().next().unwrap());
            i += 2;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    pub fn test1() {
        let mut s = "abc".to_string();
        s.insert(1, 'x');
        dbg!(s);
        assert_eq!(part1(INPUT), 1588);
    }
}
