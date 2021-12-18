use crate::day18::Number::*;
use std::collections::VecDeque;
use std::fmt::Display;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut chars: VecDeque<char> = line.chars().collect();
            Number::parse(&mut chars)
        })
        .fold(None, |left, right| match left {
            Some(l) => Some(Number::add(&l, &right)),
            None => Some(right),
        })
        .unwrap()
        .magnitude()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| {
            let mut chars: VecDeque<char> = line.chars().collect();
            Number::parse(&mut chars)
        })
        .collect();

    numbers
        .iter()
        .flat_map(|a| {
            numbers.iter().map(move |b| {
                if a == b {
                    0
                } else {
                    Number::add(a, b).magnitude()
                }
            })
        })
        .max()
        .unwrap()
} // 4709 too high

#[derive(Debug, Clone, PartialEq)]
enum Number {
    Natural(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn add(left: &Self, right: &Self) -> Self {
        let mut x = Pair(Box::new(left.clone()), Box::new(right.clone()));
        x.reduce();
        x
    }

    fn parse(mut input: &mut VecDeque<char>) -> Self {
        let next = input.pop_front().unwrap();
        if next == '[' {
            let left = Self::parse(&mut input);
            let _comma = input.pop_front().unwrap();
            let right = Self::parse(&mut input);
            let _close_bracket = input.pop_front();
            Pair(Box::new(left), Box::new(right))
        } else if next.is_numeric() {
            Natural(next.to_digit(10).unwrap() as usize)
        } else {
            panic!("Unexpected char: {}", next);
        }
    }

    fn reduce(&mut self) {
        let mut done = false;
        while !done {
            done = true;
            if let Some((_, _)) = self.reduce_explode(1) {
                done = false;
            } else if self.reduce_split() {
                done = false;
            }
        }
    }

    fn reduce_split(&mut self) -> bool {
        match self {
            Natural(_) => unreachable!(),
            Pair(ref mut left, ref mut right) => {
                let first = match **left {
                    Natural(v) => {
                        if v >= 10 {
                            let l = v / 2;
                            let r = v / 2 + v % 2;
                            *left = Box::new(Pair(Box::new(Natural(l)), Box::new(Natural(r))));
                            true
                        } else {
                            false
                        }
                    }
                    Pair(_, _) => left.reduce_split(),
                };
                if first {
                    return true;
                }

                match **right {
                    Natural(v) => {
                        if v >= 10 {
                            let l = v / 2;
                            let r = v / 2 + v % 2;
                            *right = Box::new(Pair(Box::new(Natural(l)), Box::new(Natural(r))));
                            return true;
                        }
                    }
                    Pair(_, _) => return right.reduce_split(),
                };
            }
        }

        return false;
    }
    fn reduce_explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
        match self {
            Natural(_) => None,
            Pair(ref mut left, ref mut right) => {
                if depth == 4 {
                    if let Pair(l, r) = &**left {
                        let l: usize = l.value();
                        let r: usize = r.value();
                        right.add_left_most(r);
                        *left = Box::new(Natural(0));
                        Some((Some(l), None))
                    } else if let Pair(l, r) = &**right {
                        let l: usize = l.value();
                        let r: usize = r.value();
                        left.add_right_most(l);
                        *right = Box::new(Natural(0));
                        Some((None, Some(r)))
                    } else {
                        None
                    }
                } else {
                    if let Some((to_left, to_right)) = left.reduce_explode(depth + 1) {
                        if let Some(value) = to_right {
                            right.add_left_most(value);
                        }
                        Some((to_left, None))
                    } else if let Some((to_left, to_right)) = right.reduce_explode(depth + 1) {
                        if let Some(value) = to_left {
                            left.add_right_most(value);
                        }
                        Some((None, to_right))
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn add_left_most(&mut self, value: usize) {
        match self {
            Natural(ref mut v) => *v += value,
            Pair(left, _) => left.add_left_most(value),
        }
    }

    fn add_right_most(&mut self, value: usize) {
        match self {
            Natural(ref mut v) => *v += value,
            Pair(_, right) => right.add_right_most(value),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Natural(v) => *v,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn value(&self) -> usize {
        if let Natural(v) = self {
            return *v;
        }
        panic!("Cannot get a value from a pair.");
    }
}

impl Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Natural(v) => write!(fmt, "{}", v),
            Pair(left, right) => write!(fmt, "[{},{}]", left, right),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    pub fn test1() {
        assert_eq!(part1(INPUT), 4140);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(INPUT), 3993);
    }
}
