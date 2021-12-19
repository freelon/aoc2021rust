use rustc_hash::FxHashSet;
use std::ops::Sub;

type MySet = FxHashSet<Vector>;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let scans = parse(input);
    let (beacons, _) = combine(scans);
    beacons.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let scans = parse(input);
    let (_, centers) = combine(scans);
    centers
        .iter()
        .flat_map(|a| centers.iter().map(|b| (*a - *b).len()))
        .max()
        .unwrap()
}

fn combine(mut scans: Vec<MySet>) -> (MySet, Vec<Vector>) {
    let mut beacons = scans.remove(0);
    let mut scanners = vec![Vector::new(0, 0, 0)];
    while let Some((i, (result, scanner))) = scans
        .iter()
        .enumerate()
        .flat_map(|(i, scan)| matches(&beacons, scan).map(|r| (i, r)))
        .next()
    {
        beacons = beacons.union(&result).map(|v| *v).collect();
        scans.remove(i);
        scanners.push(scanner);
    }
    print!("");
    (beacons, scanners)
}

fn matches(beacons: &MySet, other: &MySet) -> Option<(MySet, Vector)> {
    for a in beacons {
        for rotation in other.all_rotations() {
            for b in rotation.iter() {
                let diff = *b - *a;
                let realigned: MySet = rotation.iter().map(|v| *v - diff).collect();
                if beacons.intersection(&realigned).count() >= 12 {
                    return Some((realigned, Vector::new(0, 0, 0) - diff));
                }
            }
        }
    }
    None
}

trait BeaconSet {
    fn set_origin_to(&self, origin: Vector) -> Self;

    fn all_rotations(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl BeaconSet for MySet {
    fn set_origin_to(&self, origin: Vector) -> Self {
        self.iter().map(|v| *v - origin).collect()
    }

    fn all_rotations(&self) -> Vec<Self> {
        let mut result = vec![];
        for x_rot in [0, 1] {
            for y_rot in [0, 1, 2, 3] {
                for z_rot in [0, 1, 2, 3] {
                    let r = self
                        .iter()
                        .map(|v| v.rot_x_n(x_rot).rot_y_n(y_rot).rot_z_n(z_rot))
                        .collect();
                    result.push(r);
                }
            }
        }
        result
    }
}

fn parse(input: &str) -> Vec<MySet> {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let vals: Vec<_> = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
                    Vector::new(vals[0], vals[1], vals[2])
                })
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn len(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }

    fn rot_x(self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn rot_x_n(self, n: usize) -> Self {
        let mut v = self;
        for _ in 0..n {
            v = v.rot_x();
        }
        v
    }

    fn rot_y(self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rot_y_n(self, n: usize) -> Self {
        let mut v = self;
        for _ in 0..n {
            v = v.rot_y();
        }
        v
    }
    fn rot_z(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    fn rot_z_n(self, n: usize) -> Self {
        let mut v = self;
        for _ in 0..n {
            v = v.rot_z();
        }
        v
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = include!("day19_test_data.rs");

    #[test]
    pub fn test_matching() {
        let first: MySet = (0..12).map(|i| Vector::new(i, 0, 0)).collect();
        let second: MySet = (0..12)
            .map(|i| Vector::new(-13000, 200, i + 3000))
            .collect();
        let m = matches(&first, &second);
        assert_eq!(true, m.is_some());
        dbg!(m);
    }

    #[test]
    pub fn test_19_1() {
        assert_eq!(part1(INPUT), 79);
    }

    #[test]
    pub fn test_19_2() {
        assert_eq!(part2(INPUT), 3621);
    }
}
