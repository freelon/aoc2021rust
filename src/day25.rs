use rustc_hash::FxHashMap;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut map: FxHashMap<(usize, usize), char> = FxHashMap::default();
    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                map.insert((x, y), c);
            })
    });
    let w = map.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let h = map.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;

    let mut changed = true;
    let mut round = 0;
    println!("Initial state:");
    print(&map, w, h);
    while changed {
        round += 1;
        let new_map: FxHashMap<_, _> = map
            .iter()
            .map(|((x, y), c)| match *c {
                '>' => {
                    if map.contains_key(&((x + 1) % w, *y)) {
                        ((*x, *y), *c)
                    } else {
                        (((x + 1) % w, *y), *c)
                    }
                }
                'v' => ((*x, *y), *c),
                _ => panic!("unknown direction '{}'", *c),
            })
            .collect();
        let new_map: FxHashMap<_, _> = new_map
            .iter()
            .map(|((x, y), c)| match *c {
                '>' => ((*x, *y), *c),
                'v' => {
                    if new_map.contains_key(&(*x, (y + 1) % h)) {
                        ((*x, *y), *c)
                    } else {
                        (((*x), (y + 1) % h), *c)
                    }
                }
                _ => panic!("unknown direction '{}'", *c),
            })
            .collect();
        changed = new_map != map;
        map = new_map;
    }
    round
}

fn print(map: &FxHashMap<(usize, usize), char>, w: usize, h: usize) {
    (0..h).for_each(|y| {
        (0..w).for_each(|x| {
            if let Some(c) = map.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(".")
            }
        });
        println!();
    });
    println!();
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    #[test]
    pub fn test_25_1() {
        assert_eq!(part1(INPUT), 58);
    }
}
