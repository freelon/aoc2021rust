use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let neighbors = parse(input);

    find_paths(&neighbors, &mut vec!["start"], &|cave, path, _| {
        is_small_cave(cave) && path.contains(&cave)
    }, true)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let neighbors = parse(input);

    find_paths(&neighbors, &mut vec!["start"], &|cave, path, twice_used| {
        cave == "start"
            || is_small_cave(cave)
                && path.contains(&cave)
                && twice_used
    }, false)
}

fn find_paths<'a, F>(
    map: &'a HashMap<&str, Vec<&str>>,
    path_so_far: &mut Vec<&'a str>,
    visit_blocked: &F,
    twice_used: bool,
) -> usize
where
    F: Fn(&str, &Vec<&str>, bool) -> bool,
{
    let head = path_so_far.last().unwrap();

    if *head == "end" {
        return 1;
    }

    map.get(head)
        .unwrap()
        .iter()
        .map(|neighbor| {
            if visit_blocked(neighbor, &path_so_far, twice_used) {
                return 0;
            } else {
                let visiting_second_time = is_small_cave(&neighbor) && path_so_far.contains(&neighbor);
                path_so_far.push(neighbor);
                let sub_count = find_paths(map, path_so_far, visit_blocked, twice_used || visiting_second_time);
                path_so_far.pop();
                sub_count
            }
        })
        .sum()
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
