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
pub fn part2(_input: &str) -> usize {
    0
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
        assert_eq!(part2(INPUT), 3351);
    }
}
