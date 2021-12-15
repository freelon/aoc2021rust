use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Map = HashMap<Position, DangerLevel>;
type DangerLevel = usize;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let side_length = input.trim().lines().count();
    let map = parse(&input.trim());
    calc_lowest_risk_path(
        &map,
        Position::new(0, 0),
        Position::new((side_length - 1) as i32, (side_length - 1) as i32),
    )
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let side_length = input.trim().lines().count();
    let map = parse(&input.trim());

    let map = map
        .into_iter()
        .flat_map(|(p, d)| {
            (0..5).flat_map(move |x| {
                (0..5).map(move |y| {
                    let new_position =
                        Position::new(p.x + side_length as i32 * x, p.y + side_length as i32 * y);
                    let mut danger = d + x as DangerLevel + y as DangerLevel;
                    while danger > 9 {
                        danger -= 9;
                    }
                    assert_eq!(true, danger < 10);
                    (new_position, danger)
                })
            })
        })
        .collect();

    calc_lowest_risk_path(
        &map,
        Position::new(0, 0),
        Position::new((side_length * 5 - 1) as i32, (side_length * 5 - 1) as i32),
    )
}

fn calc_lowest_risk_path(map: &Map, start: Position, target: Position) -> DangerLevel {
    let mut todo = BinaryHeap::new();
    let mut already_considered = HashSet::new();
    todo.push(Reverse((0, start)));
    already_considered.insert(start);
    while let Some(Reverse((cost_so_far, current_position))) = todo.pop() {
        if current_position == target {
            return cost_so_far;
        }

        current_position
            .neighbors()
            .into_iter()
            .filter(|p| map.contains_key(p))
            .for_each(|p| {
                if !already_considered.contains(&p) {
                    already_considered.insert(p);
                    todo.push(Reverse((cost_so_far + map.get(&p).unwrap(), p)));
                }
            });
    }
    panic!("Didn't reach the target and no more paths left :(");
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Position::new(x as i32, y as i32),
                    c.to_digit(10).unwrap() as DangerLevel,
                )
            })
        })
        .collect::<Map>()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
        ]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    pub fn test1() {
        assert_eq!(part1(INPUT), 40);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(INPUT), 315);
    }
}
