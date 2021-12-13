use itertools::Itertools;
use std::collections::HashSet;

type C = usize;
type Points = HashSet<(C, C)>;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let map: Points = parts[0]
        .split("\n")
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<C>().unwrap(), y.parse::<C>().unwrap())
        })
        .collect();

    let assignments = parts[1]
        .split("\n")
        .map(|line| {
            let (axis, at) = line[11..].split_once("=").unwrap();
            (axis, at.parse::<C>().unwrap())
        })
        .collect_vec();

    fold(map, assignments[0]).len()
}

#[allow(dead_code)]
pub fn part2(input: &str) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let map: Points = parts[0]
        .split("\n")
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<C>().unwrap(), y.parse::<C>().unwrap())
        })
        .collect();

    let assignments = parts[1]
        .split("\n")
        .map(|line| {
            let (axis, at) = line[11..].split_once("=").unwrap();
            (axis, at.parse::<C>().unwrap())
        })
        .collect_vec();

    let result = assignments.into_iter().fold(map, |map, assignment| fold(map, assignment));
    print_points(&result);
}

fn print_points(map: &Points) {
        let width = map.iter().max_by_key(|(x, _)| x).unwrap().0;
    let height = map.iter().max_by_key(|(_, y)| y).unwrap().1;
    for y in 0..=height {
        for x in 0..=width {
            let c = if map.contains(&(x,y)) { "#"} else {" "};
            print!("{}", c);
        }
        println!()
    }
}

fn fold(map: Points, command: (&str, C)) -> Points {
    let (c, at) = command;
    map.into_iter()
        .map(|(x,y)| {
            match c {
                "x" => (y,x),
                "y" => (x,y),
                _ => panic!("'{}' is not a valid axis", c)
            }
        })
        .filter(|(_, y)| *y != at)
        .map(|(x, y)| if y < at { (x, y) } else { (x, 2 * at - y) })
        .map(|(x,y)| {
            match c {
                "x" => (y,x),
                "y" => (x,y),
                _ => panic!("'{}' is not a valid axis", c)
            }
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    pub fn test1() {
        assert_eq!(part1(INPUT), 17);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(INPUT), 36);
    }
}
