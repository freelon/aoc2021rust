#![allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (template, rules) = input.trim().split_once("\n\n").unwrap();
    let rules: HashMap<&str, &str> = rules
        .split("\n")
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();

    let mut polymer: String = template.to_string();
    for _i in 0..10 {
        polymer = fast(polymer, &rules);
    }
    let frequencies = polymer.chars().counts();
    let hfreq = frequencies.iter().max_by_key(|(_k, v)| *v).unwrap();
    let lfreq = frequencies.iter().min_by_key(|(_k, v)| *v).unwrap();

    hfreq.1 - lfreq.1
}

fn fast(poly: String, rules: &HashMap<&str, &str>) -> String {
    let mut back = VecDeque::new();
    let mut coming = poly.chars();
    let first = coming.next().unwrap();
    back.push_back(first);
    while let Some(head) = coming.next() {
        let part = &format!("{}{}", back.get(back.len() - 1).unwrap(), head);
        if let Some(c) = rules.get(part.as_str()) {
            back.push_back(c.chars().next().unwrap());
        }
        back.push_back(head);
    }
    back.into_iter().collect()
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
