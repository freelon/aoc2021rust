use rustc_hash::FxHashMap;

type Image = FxHashMap<(i32, i32), char>;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    run(input, 2)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    run(input, 50)
}

fn run(input: &str, factor: i32) -> usize {
    let (rule, mut image) = parse(input);

    let width = image.keys().max_by_key(|(x, _)| x).unwrap().0;
    let height = image.keys().max_by_key(|(_, y)| y).unwrap().1;

    for x in -factor - 1..width + factor + 1 {
        for y in -factor - 1..height + factor + 1 {
            if !image.contains_key(&(x, y)) {
                image.insert((x, y), '.');
            }
        }
    }

    (0..factor)
        .fold(image, |image, _| apply(image, &rule))
        .values()
        .filter(|c| **c == '#')
        .count()
}

fn apply(image: Image, rule: &Vec<char>) -> Image {
    image
        .iter()
        .map(|((px, py), c)| {
            let ns: String = (py - 1..=py + 1)
                .flat_map(|y| {
                    (px - 1..=px + 1)
                        .map(|x| image.get(&(x, y)).unwrap_or(c))
                        .collect::<Vec<_>>()
                })
                .map(|c| match c {
                    '#' => '1',
                    '.' => '0',
                    _ => panic!("illegal char {}", c),
                })
                .collect();
            let rule_index = usize::from_str_radix(&ns, 2).unwrap();
            ((*px, *py), rule[rule_index])
        })
        .collect()
}

fn parse(input: &str) -> (Vec<char>, Image) {
    let (rule, image) = input.trim().split_once("\n\n").unwrap();
    let rule = rule.chars().collect();
    let image = image
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();
    (rule, image)
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    pub fn test_20_1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    pub fn test_20_2() {
        assert_eq!(part2(INPUT), 3351);
    }
}
