use std::cmp::max;
use std::cmp::min;
use std::ops::RangeInclusive;

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let ((x_start, x_end), (y_start, y_end)) = parse(input);

    (0..=x_end)
        .flat_map(|vx| {
            (y_end..=x_end)
                .map(move |vy| hits((vx, vy), (x_start, y_end), (x_end, y_start)))
                .flat_map(|o| o)
        })
        .max()
        .unwrap()
}

fn parse(input: &str) -> ((i32, i32), (i32, i32)) {
    let (a, b) = input
        .trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (y_start, y_end) = b[2..].split_once("..").unwrap();
    let (x_start, x_end) = a[2..].split_once("..").unwrap();

    let (x_start, x_end) = (
        x_start.parse::<i32>().unwrap(),
        x_end.parse::<i32>().unwrap(),
    );
    let (y_start, y_end) = (
        y_start.parse::<i32>().unwrap(),
        y_end.parse::<i32>().unwrap(),
    );
    ((x_start, x_end), (y_start, y_end))
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let ((x_start, x_end), (y_start, y_end)) = parse(input);

    (0..=x_end)
        .flat_map(|vx| {
            (y_start..=x_end)
                .map(move |vy| hits((vx, vy), (x_start, y_end), (x_end, y_start)))
                .flat_map(|o| o)
        })
        .count()
}

type V = (i32, i32);

fn hits(mut v: V, top_left: V, bottom_right: V) -> Option<i32> {
    let mut probe = (0, 0);
    let mut max_y = 0;
    while probe.0 < bottom_right.0 && probe.1 > bottom_right.1 {
        probe = (probe.0 + v.0, probe.1 + v.1);
        max_y = max(max_y, probe.1);

        if range(top_left.0, bottom_right.0).contains(&probe.0)
            && range(top_left.1, bottom_right.1).contains(&probe.1)
        {
            return Some(max_y);
        }

        let dx = if v.0 > 0 {
            -1
        } else if v.0 < 0 {
            1
        } else {
            0
        };
        v = (v.0 + dx, v.1 - 1);
    }
    return None;
}

fn range(a: i32, b: i32) -> RangeInclusive<i32> {
    min(a, b)..=max(a, b)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(part1("target area: x=20..30, y=-10..-5"), 45);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2("target area: x=20..30, y=-10..-5"), 112);
    }
}
