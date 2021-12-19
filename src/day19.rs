use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let scans = parse(input);
    let beacons = combine(scans);
    beacons.len()
}

fn combine(mut scans: Vec<HashSet<Vector>>) -> HashSet<Vector> {
    let mut beacons = scans.remove(0);
    while let Some((i, result)) = scans
        .iter()
        .enumerate()
        .flat_map(|(i, scan)| matches(&beacons, scan).map(|r| (i, r)))
        .next()
    {
        beacons = beacons.union(&result).map(|v| *v).collect();
        scans.remove(i);
    }
    print!("");
    beacons
}

fn matches(beacons: &HashSet<Vector>, other: &HashSet<Vector>) -> Option<HashSet<Vector>> {
    for a in beacons {
        for rotation in other.all_rotations() {
            for b in rotation.iter() {
                let diff = b.sub(*a);
                let realigned: HashSet<_> = rotation.iter().map(|v| v.sub(diff)).collect();
                if beacons.intersection(&realigned).count() >= 12 {
                    return Some(realigned);
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

impl BeaconSet for HashSet<Vector> {
    fn set_origin_to(&self, origin: Vector) -> Self {
        self.iter().map(|v| v.sub(origin)).collect()
    }

    fn all_rotations(&self) -> Vec<Self> {
        let mut all_expanded: Vec<_> = self.iter().map(|v| v.all_rotations()).collect();
        let perspectives = all_expanded[0].len();
        (0..perspectives)
            .map(|_| {
                all_expanded
                    .iter_mut()
                    .map(|vp| vp.pop().unwrap())
                    .collect()
            })
            .collect()
    }
}

fn parse(input: &str) -> Vec<HashSet<Vector>> {
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

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn all_rotations(&self) -> Vec<Self> {
        let Self { x, y, z } = self;
        [
            Self {
                x: *x,
                y: *z,
                z: *y,
            },
            Self {
                x: *x,
                y: *y,
                z: *z,
            },
            Self {
                x: *y,
                y: *x,
                z: *z,
            },
            Self {
                x: *y,
                y: *z,
                z: *x,
            },
            Self {
                x: *z,
                y: *y,
                z: *x,
            },
            Self {
                x: *z,
                y: *x,
                z: *y,
            },
        ]
        .into_iter()
        .flat_map(|Self { x, y, z }| {
            [
                Self { x: x, y: y, z: z },
                Self { x: x, y: y, z: -z },
                Self { x: x, y: -y, z: z },
                Self { x: x, y: -y, z: -z },
                Self { x: -x, y: y, z: z },
                Self { x: -x, y: y, z: -z },
                Self { x: -x, y: -y, z: z },
                Self {
                    x: -x,
                    y: -y,
                    z: -z,
                },
            ]
        })
        .collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: &str = include!("day19_test_data.rs");

    #[test]
    pub fn test_matching() {
        let first: HashSet<_> = (0..12).map(|i| Vector::new(i, 0, 0)).collect();
        let second: HashSet<_> = (0..12)
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
    pub fn test2() {
        // assert_eq!(part2(INPUT), 3993);
    }
}
