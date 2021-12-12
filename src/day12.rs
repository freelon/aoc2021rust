use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let l: Vec<&str> = line.split("-").collect();
        let a = l.get(0).unwrap().to_string();
        let b = l.get(1).unwrap().to_string();
        if !neighbors.contains_key(&a) {
            neighbors.insert(a.clone(), vec![]);
        }
        if !neighbors.contains_key(&b) {
            neighbors.insert(b.clone(), vec![]);
        }

        let la = neighbors.get_mut(&a).unwrap();
        la.push(b.clone());
        let lb = neighbors.get_mut(&b).unwrap();
        lb.push(a.clone());
    });

    println!("Map: {:?}", neighbors);

    find_paths(&neighbors, vec!["start".to_owned()])
        .iter()
        .count()
}

fn find_paths(
    map: &HashMap<String, Vec<String>>,
    path_so_far: Vec<String>,
) -> HashSet<Vec<String>> {
    let head = path_so_far.last().unwrap();

    if head == "end" {
        let mut result = HashSet::new();
        result.insert(path_so_far);
        return result;
    }

    map.get(head).unwrap().iter().flat_map(|neighbor| {
        // only add capitals more than once
        if is_small_cave(neighbor) && path_so_far.contains(neighbor) {
            return HashSet::new();
        } else {
            let mut path = path_so_far.clone();
            path.push(neighbor.to_owned());
            return find_paths(map, path);
        }
    })
    .collect()
}

fn is_small_cave(cave: &str) -> bool {
    cave == cave.to_lowercase()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    #[test]
    pub fn test1() {
        assert_eq!(part1(INPUT), 10);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(INPUT), 133742);
    }
}
