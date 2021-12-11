use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

type State = HashMap<Position, Counter>;
type Counter = usize;

pub fn part1(input: &str) -> Counter {
    let state = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Position::new(x as i32, y as i32),
                    c.to_digit(10).unwrap() as Counter,
                )
            })
        })
        .collect::<State>();
    (1..=100)
        .fold((state, 0), |(state, flashes), r| {
            log::debug!("(before) Round {}", r);
            print(&state);

            let (state_after, new_flashes) = round(state);
            (state_after, flashes + new_flashes)
        })
        .1
}

fn round(start: State) -> (State, Counter) {
    let mut state = start;
    let mut to_increment: Vec<Position> = state.keys().cloned().collect();
    let mut flashed: HashSet<Position> = HashSet::new();
    while !to_increment.is_empty() {
        for p in to_increment {
            if let Some(v) = state.get_mut(&p) {
                *v += 1;
            }
        }
        let flashing: HashSet<Position> = state
            .iter()
            .filter(|(pos, v)| **v > 9 && !flashed.contains(pos))
            .map(|(pos, _)| *pos)
            .collect();

        log::debug!("flashing: {:?}", flashing);

        to_increment = flashing
            .iter()
            .flat_map(|p| p.neighbors())
            .filter(|p| state.contains_key(p))
            .collect();
        flashed.extend(flashing);
    }

    (
        state
            .into_iter()
            .map(|(p, v)| (p, if v > 9 { 0 } else { v }))
            .collect(),
        flashed.len(),
    )
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| Position::new(self.x + x, self.y + y)))
            .filter(|p| p.x != self.x || p.y != self.y)
            .collect()
    }
}

fn print(state: &State) {
    state
        .keys()
        .sorted_by_key(|Position { x: _, y }| y)
        .group_by(|Position { x: _, y }| y)
        .into_iter()
        .for_each(|(y, positions)| {
            let s = positions
                .sorted_by_key(|Position { x, y: _ }| *x)
                .map(|p| state.get(p).unwrap().to_string())
                .collect::<String>();
            log::debug!("line {}: {}", y, s);
        });
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    pub fn test1() {
        assert_eq!(part1(INPUT), 1656);
    }
}
