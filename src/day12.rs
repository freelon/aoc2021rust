use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let neighbors = parse(input);

    find_paths(&neighbors, vec!["start"], &|cave, path| {
        is_small_cave(cave) && path.contains(&cave)
    })
    .iter()
    .count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let neighbors = parse(input);

    find_paths(&neighbors, vec!["start"], &|cave, path| {
        cave == "start"
            || is_small_cave(cave)
                && path.contains(&cave)
                && path
                    .iter()
                    .filter(|c| is_small_cave(c))
                    .any(|c| path.iter().filter(|v| *v == c).count() > 1)
    })
    .iter()
    .count()
}

fn find_paths<'a, F>(
    map: &'a HashMap<&str, Vec<&str>>,
    path_so_far: Vec<&'a str>,
    visit_blocked: &F,
) -> HashSet<Vec<&'a str>>
where
    F: Fn(&str, &Vec<&str>) -> bool,
{
    let head = path_so_far.last().unwrap();

    if *head == "end" {
        let mut result = HashSet::<Vec<&str>>::new();
        result.insert(path_so_far.iter().map(|cave| *cave).collect());
        return result;
    }

    map.get(head)
        .unwrap()
        .iter()
        .flat_map(|neighbor| {
            if visit_blocked(neighbor, &path_so_far) {
                return HashSet::new();
            } else {
                let mut path = path_so_far.clone();
                path.push(neighbor.to_owned());
                return find_paths(map, path, visit_blocked);
            }
        })
        .collect()
}

fn is_small_cave(cave: &str) -> bool {
    cave == cave.to_lowercase()
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let l: Vec<&str> = line.split("-").collect();
        let a = l.get(0).unwrap();
        let b = l.get(1).unwrap();
        if !neighbors.contains_key(a) {
            neighbors.insert(a, vec![]);
        }
        if !neighbors.contains_key(b) {
            neighbors.insert(b, vec![]);
        }

        let la = neighbors.get_mut(a).unwrap();
        la.push(b);
        let lb = neighbors.get_mut(b).unwrap();
        lb.push(a);
    });

    neighbors
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
        assert_eq!(part2(INPUT), 36);
    }
}
