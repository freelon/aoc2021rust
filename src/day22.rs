use rustc_hash::FxHashSet;
use std::cmp::max;
use std::cmp::min;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let v: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (command, coords) = line.split_once(" ").unwrap();
            let coords: Vec<_> = coords
                .split(",")
                .map(|fromto| {
                    let (_, c) = fromto.split_once("=").unwrap();
                    let (from, to) = c.split_once("..").unwrap();
                    (from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap())
                })
                .collect();
            (command, coords)
        })
        .collect();

    let mut on: FxHashSet<(i32, i32, i32)> = FxHashSet::default();
    for (command, coords) in &v {
        for x in coords[0].0..=coords[0].1 {
            if x < -50 || x > 50 {
                continue;
            }

            for y in coords[1].0..=coords[1].1 {
                if y < -50 || y > 50 {
                    continue;
                }

                for z in coords[2].0..=coords[2].1 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    if *command == "on" {
                        on.insert((x, y, z));
                    } else if *command == "off" {
                        on.remove(&(x, y, z));
                    } else {
                        panic!("illegal command '{}'", command);
                    }
                }
            }
        }
    }
    on.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let v: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (command, coords) = line.split_once(" ").unwrap();
            let coors: Vec<_> = coords
                .split(",")
                .map(|fromto| {
                    let (_, c) = fromto.split_once("=").unwrap();
                    let (from, to) = c.split_once("..").unwrap();
                    (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap())
                })
                .collect();
            let cube = Cube::new(
                coors[0].0, coors[0].1, coors[1].0, coors[1].1, coors[2].0, coors[2].1,
            );
            (command, cube)
        })
        .collect();

        println!("using {} inputs", v.len());

    v.into_iter()
        .fold(vec![], |cubes: Vec<Cube>, (command, cube)| {
            let mut result: Vec<_> = cubes
                .into_iter()
                .flat_map(|other| other.bite(cube))
                .collect();
            if command == "on" {
                result.push(cube);
            }
            result
        })
        .into_iter()
        .map(|cube| cube.len())
        .sum()
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Range { start, end }
    }

    fn len(&self) -> usize {
        if self.end >= self.start {
            (self.end - self.start + 1) as usize
        } else {
            0
        }
    }

    fn empty(&self) -> bool {
        self.end < self.start
    }

    fn overlaps(&self, o: Self) -> bool {
        self.contains(o.start)
            || self.contains(o.end)
            || o.contains(self.start)
            || o.contains(self.end)
    }

    fn contains(&self, v: i64) -> bool {
        self.start <= v && self.end >= v
    }
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    ax: Range,
    ay: Range,
    az: Range,
}

impl Cube {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Self {
        Cube {
            ax: Range::new(x1, x2),
            ay: Range::new(y1, y2),
            az: Range::new(z1, z2),
        }
    }

    fn bite(self, other: Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![self];
        }

        let xa = self.ax.start;
        let xb = min(other.ax.start - 1, self.ax.end);
        let xb_start = max(xa, xb + 1);
        let xc = min(other.ax.end, self.ax.end);
        let xc_start = xc + 1;
        let xd = self.ax.end;

        let ya = self.ay.start;
        let yb = min(other.ay.start - 1, self.ay.end);
        let yb_start = max(ya, yb+1);
        let yc = min(other.ay.end, self.ay.end);
        let yc_start = yc + 1;
        let yd = self.ay.end;

        let za = self.az.start;
        let zb = min(other.az.start - 1, self.az.end);
        let _zb_start = max(za, zb +1);
        let zc = min(other.az.end, self.az.end);
        let zc_start = zc + 1;
        let zd = self.az.end;

        let a = Cube {
            ax: Range { start: xa, end: xb },
            ..self
        };
        let b = Cube {
            ax: Range {
                start: xb_start,
                end: xc,
            },
            ay: Range { start: ya, end: yb },
            ..self
        };
        let c = Cube {
            ax: Range {
                start: xb_start,
                end: xc,
            },
            ay: Range {
                start: yb_start,
                end: yc,
            },
            az: Range { start: za, end: zb },
        };
        let d = Cube {
            ax: Range {
                start: xb_start,
                end: xc,
            },
            ay: Range {
                start: yb_start,
                end: yc,
            },
            az: Range {
                start: zc_start,
                end: zd,
            },
        };
        let e = Cube {
            ax: Range {
                start: xb_start,
                end: xc,
            },
            ay: Range {
                start: yc_start,
                end: yd,
            },
            ..self
        };
        let f = Cube {
            ax: Range {
                start: xc_start,
                end: xd,
            },
            ..self
        };

        vec![a, b, c, d, e, f]
            .into_iter()
            .filter(|cube| !cube.empty())
            .collect()
    }

    fn overlaps(self, other: Self) -> bool {
        self.ax.overlaps(other.ax) && self.ay.overlaps(other.ay) && self.az.overlaps(other.az)
    }

    fn empty(&self) -> bool {
        self.ax.empty() || self.ay.empty() || self.az.empty()
    }

    fn len(&self) -> usize {
        self.ax.len() * self.ay.len() * self.az.len()
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    const INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

    #[test]
    pub fn bla() {
        let input = "on x=3..7,y=0..4,z=0..4
on x=0..7,y=0..2,z=0..4";
// let input = "on x=-20..26,y=-36..17,z=-47..7
// on x=-39..-5,y=-6..47,z=-3..44
// ";
        assert_eq!(part1(input), part2(input));
    }

    #[test]
    pub fn test_22_1() {
        assert_eq!(part1(INPUT), 590784);
    }

    #[test]
    pub fn test_22_2() {
        assert_eq!(part2("on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
"), 2758514936282235);
    }
}
