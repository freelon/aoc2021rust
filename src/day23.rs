use crate::day15::Position;
use human_format::Formatter;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::fmt::Debug;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let start = State { map, cost: 0 };
    solve(start).cost
}

fn solve(state: State) -> State {
    let mut open: PriorityQueue<Map, Reverse<usize>> = PriorityQueue::new();
    let mut inspection: FxHashSet<Map> = FxHashSet::default();
    open.push_increase(state.map, Reverse(state.cost));

    let mut counter = 0;
    while let Some((next_map, next_cost)) = open.pop() {
        let next = State {
            map: next_map,
            cost: next_cost.0,
        };
        counter += 1;
        if counter % 10_000 == 0 {
            println!(
                "Visited {}, current cost: {} (queue size {})",
                Formatter::new().with_decimals(1).format(counter as f64),
                next_cost.0,
                Formatter::new().with_decimals(1).format(open.len() as f64)
            );
        }
        if next.is_solved() {
            return next;
        }

        if inspection.contains(&next.map) {
            continue;
        }

        next.follow_ups().into_iter().for_each(|f| {
            open.push_increase(f.map, Reverse(f.cost));
        });
        inspection.insert(next.map.clone());
    }

    panic!("No more states to search");
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| (Position::new(x as i32, y as i32), c))
        })
        .fold(Map::default(), |mut map, (position, c)| {
            map[position.x as usize][position.y as usize] = c;
            map
        })
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> usize {
    0
}

const HEIGHT: usize = 7;
const WIDTH: usize = 14;
type Map = [[u8; HEIGHT]; WIDTH];

#[derive(Clone, Hash)]
struct State {
    map: Map,
    cost: usize,
}

const TARGETS: [(i32, u8); 4] = [(3, b'A'), (5, b'B'), (7, b'C'), (9, b'D')];

impl State {
    fn get(&self, p: Position) -> u8 {
        self.map[p.x as usize][p.y as usize]
    }
    fn set(&mut self, p: Position, c: u8) {
        self.map[p.x as usize][p.y as usize] = c;
    }

    fn is_solved(&self) -> bool {
        if (1..WIDTH).any(|x| self.get(Position::new(x as i32, 1)).is_ascii_alphabetic()) {
            return false;
        }
        for (x, c) in TARGETS {
            if (2..HEIGHT).any(|y| {
                let occ = self.get(Position::new(x, y as i32));
                occ.is_ascii_alphabetic() && occ != c
            }) {
                return false;
            }
        }
        true
    }

    fn follow_ups(&self) -> Vec<Self> {
        (0..WIDTH)
            .flat_map(|x| {
                (0..HEIGHT).map(move |y| (Position::new(x as i32, y as i32), self.map[x][y]))
            })
            .filter(|(_, c)| c.is_ascii_alphabetic())
            .flat_map(|(p, c)| self.follow_upss(p, c))
            .collect()
    }

    fn field_allowed(&self, _from: Position, to: Position) -> bool {
        if self.get(to) != b'.' {
            return false;
        }

        return true;
    }

    fn follow_upss(&self, p: Position, c: u8) -> Vec<Self> {
        let mut visited: FxHashMap<Position, usize> = FxHashMap::default();
        let mut open: Vec<(Position, usize)> = p
            .neighbors()
            .into_iter()
            .filter(|np| self.field_allowed(p, *np))
            .map(|np| (np, 1))
            .collect();

        while let Some((next, csf)) = open.pop() {
            let nexts: Vec<_> = next
                .neighbors()
                .into_iter()
                .filter(|np| self.field_allowed(p, *np))
                .filter(|np| !open.iter().any(|it| it.0 == *np) && !visited.contains_key(&np))
                .map(|np| (np, csf + 1))
                .collect();
            for np in nexts {
                open.push(np);
            }
            visited.insert(next, csf);
        }

        visited
            .into_iter()
            .filter(|(np, _)| self.valid_target(&p, np, c))
            .map(|(np, dist)| {
                let cost = dist
                    * match c {
                        b'A' => 1,
                        b'B' => 10,
                        b'C' => 100,
                        b'D' => 1000,
                        _ => panic!("Cannot move walls or other things"),
                    };
                let mut new_state = State {
                    map: self.map.clone(),
                    cost: self.cost + cost,
                };
                new_state.set(np, c);
                new_state.set(p, b'.');
                new_state
            })
            .collect()
    }

    fn valid_target(&self, from: &Position, to: &Position, c: u8) -> bool {
        if from.y == 1 && to.y == 1 {
            return false;
        }

        if to.y == 1 && !TARGETS.iter().filter(|it| it.0 == to.x).next().is_some() {
            return true;
        }
        if to.y > 1 {
            if let Some((col, d)) = TARGETS.iter().filter(|it| it.0 == to.x).next() {
                if (2..=5).any(|row| {
                    let occupant = self.map[*col as usize][row as usize];
                    (occupant.is_ascii_alphabetic() && occupant != *d)
                        || (row > to.y && occupant == b'.')
                }) {
                    return false;
                }

                if *d == c {
                    return true;
                }
            }
        }

        false
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let c = self.get(Position::new(x as i32, y as i32));
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Cost so far: {}", self.cost)
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########

";
    #[test]
    pub fn test_23_1() {
        assert_eq!(part1(INPUT), 12521);
    }

    #[test]
    fn follow_ups() {
        let i = "#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########
";
        State {
            map: parse(i),
            cost: 0,
        }
        .follow_ups()
        .into_iter()
        .for_each(|it| println!("{:?}", it));
    }

    #[test]
    pub fn test_23_2() {
        let input2 = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";
        assert_eq!(part1(input2), 44169);
    }
}