use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

type State = HashMap<Position, Counter>;
type Counter = usize;

#[allow(dead_code)]
pub fn part1(input: &str) -> Counter {
    let state = parse(input);
    (1..=100)
        .fold((state, 0), |(state, flashes), r| {
            log::debug!("(before) Round {}", r);
            print(&state);

            let (state_after, new_flashes) = play_round(state);
            (state_after, flashes + new_flashes)
        })
        .1
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let state = parse(input);
    (1..)
        .fold_while((state, 0), |(state, _), round| {
            let (s, flashing) = play_round(state);
            if s.len() == flashing {
                Done((s, round))
            } else {
                Continue((s, 0))
            }
        })
        .into_inner()
        .1
}

fn play_round(start: State) -> (State, Counter) {
    let all = start.keys().cloned().collect();
    play(start, all, HashSet::new())
}

fn play(mut state: State, to_increment: Vec<Position>, mut flashed: HashSet<Position>) -> (State, Counter) {
    if to_increment.is_empty() {
        return (
            state
                .into_iter()
                .map(|(p, v)| (p, if v > 9 { 0 } else { v }))
                .collect(),
            flashed.len(),
        );
    }

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

    let to_increment = flashing
        .iter()
        .flat_map(|p| p.neighbors())
        .filter(|p| state.contains_key(p))
        .collect();
    flashed.extend(flashing);

    play(state, to_increment, flashed)
}

fn parse(input: &str) -> State {
    input
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
        .collect::<State>()
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

    #[test]
    pub fn test2() {
        assert_eq!(part2(INPUT), 195);
    }
}
