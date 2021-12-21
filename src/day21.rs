use rustc_hash::FxHashMap;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let v: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (loose, dice_count) = play(&v);
    loose * dice_count
}

fn play(v: &Vec<usize>) -> (usize, usize) {
    let mut dice = 0;
    let mut throw_count = 0;
    let mut state: Vec<(usize, usize)> = v.iter().map(|start| (*start, 0)).collect();
    let mut turn = 0;
    loop {
        let mut to_add: usize = 0;
        for _ in 1..=3 {
            throw_count += 1;
            dice += 1;
            if dice == 101 {
                dice = 1;
            }
            to_add += dice;
        }

        if let Some(blubb) = state.get_mut(turn) {
            let field = &mut blubb.0;
            let score = &mut blubb.1;
            *field = *field + to_add;
            while *field > 10 {
                *field -= 10;
            }
            *score += *field;
            if *score >= 1000 {
                return (state[(turn + 1) % 2].1, throw_count);
            }
        }
        turn = (turn + 1) % 2;
    }
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let v: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let target = TurnResult {
        pos1: v[0] as i8,
        pos2: v[1] as i8,
        points1: 0,
        points2: 0,
    };
    let mut cache = FxHashMap::<TurnResult, (usize, usize)>::default();
    let result = run(target, &mut cache);
    std::cmp::max(result.0, result.1)
}

fn run(state: TurnResult, mut cache: &mut FxHashMap<TurnResult, (usize, usize)>) -> (usize, usize) {
    if let Some(win) = state.wins() {
        return win;
    }

    if let Some(result) = cache.get(&state) {
        return *result;
    }

    let result = (3..=9).fold((0, 0), |(acc1, acc2), step_length| {
        let mut new = state.clone();
        new.pos1 += step_length;
        if new.pos1 > 10 {
            new.pos1 %= 10;
        }
        new.points1 += new.pos1;

        let (a1, a2) = run(new.flip(), &mut cache);
        let multiplicator = ROLL_DISTRIBUTION[step_length as usize];
        (acc1 + multiplicator * a2, acc2 + multiplicator * a1)
    });
    cache.insert(state, result);
    result
}

const ROLL_DISTRIBUTION: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct TurnResult {
    pos1: i8,
    pos2: i8,
    points1: i8,
    points2: i8,
}

impl TurnResult {
    fn wins(&self) -> Option<(usize, usize)> {
        if self.points1 >= 21 {
            return Some((1, 0));
        }
        if self.points2 >= 21 {
            return Some((0, 1));
        }
        None
    }

    fn flip(&self) -> Self {
        TurnResult {
            pos1: self.pos2,
            pos2: self.pos1,
            points1: self.points2,
            points2: self.points1,
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    pub fn test_21_1() {
        assert_eq!(part1(INPUT), 739785);
    }

    #[test]
    pub fn test_21_2() {
        assert_eq!(part2(INPUT), 444356092776315);
    }
}
