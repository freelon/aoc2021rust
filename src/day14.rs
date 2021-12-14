use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn both_parts(input: &str, rounds: usize) -> usize {
    let (template, rules) = input.trim().split_once("\n\n").unwrap();
    let rules: HashMap<String, char> = rules
        .split("\n")
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(i, o)| (i.to_string(), o.chars().next().unwrap()))
        .collect();

    let initial = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|(a, b)| format!("{}{}", a, b))
        .counts();

    let pair_frequencies = process(initial, rules, rounds);
    let mut frequencies: HashMap<char, usize> = pair_frequencies
        .into_iter()
        .flat_map(|(pair, pair_count)| {
            vec![
                (pair.chars().next().unwrap(), pair_count),
                (pair.chars().skip(1).next().unwrap(), pair_count),
            ]
        })
        .sorted()
        .group_by(|(character, _count)| *character)
        .into_iter()
        .map(|(ch, counts)| (ch, counts.into_iter().map(|(_, count)| count).sum()))
        .collect();

    let first_char = template.chars().next().unwrap();
    let second_char = template.chars().last().unwrap();
    *(frequencies.get_mut(&first_char).unwrap()) += 1;
    *(frequencies.get_mut(&second_char).unwrap()) += 1;
    frequencies.iter_mut().for_each(|(_, count)| *count = *count / 2);

    let hfreq = frequencies.iter().max_by_key(|(_k, v)| *v).unwrap();
    let lfreq = frequencies.iter().min_by_key(|(_k, v)| *v).unwrap();

    hfreq.1 - lfreq.1
}

fn process(
    pairs: HashMap<String, usize>,
    rules: HashMap<String, char>,
    height: usize,
) -> HashMap<String, usize> {
    if height == 0 {
        return pairs;
    }

    let mut neu: HashMap<String, usize> = HashMap::new();

    for (pair, count) in pairs {
        if let Some(c) = rules.get(&pair) {
            let triple: String = format!("{}{}{}", &pair[0..1], c, &pair[1..=1]);
            let left_pair: String = format!("{}", &triple[0..=1]);
            let right_pair: String = format!("{}", &triple[1..=2]);

            if !neu.contains_key(&left_pair) {
                neu.insert(left_pair.clone(), 0);
            }
            *(neu.get_mut(&left_pair).unwrap()) += count;

            if !neu.contains_key(&right_pair) {
                neu.insert(right_pair.clone(), 0);
            }
            *(neu.get_mut(&right_pair).unwrap()) += count;
        } else {
            if !neu.contains_key(&pair) {
                neu.insert(pair.clone(), 0);
            }
            *(neu.get_mut(&pair).unwrap()) += count;
        }
    }

    process(neu, rules, height - 1)
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
        assert_eq!(both_parts(INPUT, 10), 1588);
    }

    #[test]
    pub fn test2() {
        assert_eq!(both_parts(INPUT, 40), 2188189693529);
    }
}
