use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use advent::input_store;
use itertools::Itertools;
use vecmath::{col_mat3_transform, row_mat3_mul, vec3_sub, Matrix3, Vector3};

#[derive(Debug, Clone)]
struct ScannerPerspective {
    beacons: HashSet<Vector3<isize>>,
    distance_map: HashMap<Vector3<isize>, Vec<isize>>,
}

fn compare_vecs(a_vec: &Vec<isize>, b_vec: &Vec<isize>) -> usize {
    let mut total = 0;

    let mut a_iter = a_vec.iter();
    let mut b_iter = b_vec.iter();

    let mut a_num = a_iter.next();
    let mut b_num = b_iter.next();

    loop {
        if a_num.is_none() || b_num.is_none() {
            break;
        }

        if a_num.is_some() && a_num == b_num {
            total += 1;
            a_num = a_iter.next();
            b_num = b_iter.next();
        } else {
            if a_num > b_num {
                b_num = b_iter.next();
            } else {
                a_num = a_iter.next();
            }
        }
    }

    total
}

impl ScannerPerspective {
    fn similar(&self, other: &Self) -> Vec<(Vector3<isize>, Vector3<isize>, usize)> {
        let mut results = Vec::new();
        for (source, source_dmap) in &self.distance_map {
            for (test, test_dmap) in &other.distance_map {
                let matched = compare_vecs(source_dmap, test_dmap);

                // let matched = source_dmap
                //     .iter()
                //     .zip(test_dmap.iter())
                //     .filter(|(&a, &b)| a == b)
                //     .count();
                if matched >= 6 {
                    results.push((source.clone(), test.clone(), matched));
                }
            }
        }

        results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        results.reverse();

        results
    }

    fn transform(&self, delta: Vector3<isize>) -> Self {
        let beacons = self.beacons.iter().map(|&v| vec3_sub(v, delta)).collect();
        Self {
            beacons,
            distance_map: self.distance_map.clone(),
        }
    }
}

impl From<HashSet<Vector3<isize>>> for ScannerPerspective {
    fn from(beacons: HashSet<Vector3<isize>>) -> Self {
        let mut distance_map = HashMap::new();
        for this in &beacons {
            let mut distances = Vec::new();
            for other in &beacons {
                let mag = vec3_sub(*this, *other);
                let dist = mag[0].abs() + mag[1].abs() + mag[2].abs();
                distances.push(dist);
                if distances.len() > 20 {
                    break;
                }
            }
            distances.sort();

            distance_map.insert(*this, distances);
        }

        Self {
            beacons,
            distance_map,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    perspectives: Vec<ScannerPerspective>,
}

impl From<&str> for Scanner {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next();
        let beacons: HashSet<Vector3<isize>> = lines
            .map(|l| {
                let val: Vec<isize> = l
                    .trim()
                    .split(",")
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                [val[0usize], val[1usize], val[2usize]]
            })
            .collect();

        beacons.into()
    }
}

impl From<HashSet<Vector3<isize>>> for Scanner {
    fn from(beacons: HashSet<Vector3<isize>>) -> Self {
        let mut perspectives = Vec::new();

        for rm in all_rotations() {
            let rotated: HashSet<Vector3<isize>> = beacons.iter().map(|v| rotate(rm, *v)).collect();
            perspectives.push(rotated.into());
        }

        Self { perspectives }
    }
}

impl Scanner {
    fn merge(&self, other: &Self) -> Option<Self> {
        for our_perspective in &self.perspectives {
            for their_perspective in &other.perspectives {
                let similars = our_perspective.similar(&their_perspective);
                if similars.len() == 0 {
                    continue;
                }

                for (absolute, relative, _) in similars {
                    let delta = vec3_sub(relative, absolute);
                    let transformed_relative = their_perspective.transform(delta);

                    let intersection: HashSet<_> = transformed_relative
                        .beacons
                        .intersection(&our_perspective.beacons)
                        .collect();

                    if intersection.len() >= 12 {
                        println!("would combine");

                        let combined: HashSet<_> = our_perspective
                            .beacons
                            .union(&transformed_relative.beacons)
                            .cloned()
                            .collect();
                        return Some(combined.into());
                    }
                }
            }
        }

        None
    }
}

fn rotate(rm: Matrix3<f32>, point: Vector3<isize>) -> Vector3<isize> {
    let point = [point[0] as f32, point[1] as f32, point[2] as f32];

    let rotated = col_mat3_transform(rm, point);

    [
        rotated[0] as isize,
        rotated[1] as isize,
        rotated[2] as isize,
    ]
}

fn rotation_x(deg: f32) -> Matrix3<f32> {
    [
        [1., 0., 0.],
        [0., deg.cos(), deg.sin()],
        [0., -deg.sin(), deg.cos()],
    ]
}

fn rotation_y(deg: f32) -> Matrix3<f32> {
    [
        [deg.cos(), 0., -deg.sin()],
        [0., 1., 0.],
        [deg.sin(), 0., deg.cos()],
    ]
}

fn rotation_z(deg: f32) -> Matrix3<f32> {
    [
        [deg.cos(), deg.sin(), 0.],
        [-deg.sin(), deg.cos(), 0.],
        [0., 0., 1.],
    ]
}

enum Axis {
    X,
    Y,
    Z,
}

impl From<char> for Axis {
    fn from(c: char) -> Self {
        match c {
            'x' | 'X' => Self::X,
            'y' | 'Y' => Self::Y,
            'z' | 'Z' => Self::Z,
            _ => unreachable!(),
        }
    }
}

impl Axis {
    fn rotation_matrix(&self) -> Matrix3<f32> {
        match self {
            Axis::X => rotation_x(PI / 2.),
            Axis::Y => rotation_y(PI / 2.),
            Axis::Z => rotation_z(PI / 2.),
        }
    }
}

struct Rotations(Vec<Axis>);

impl Rotations {
    fn reduce(&self) -> Matrix3<f32> {
        self.0
            .iter()
            .map(|axis| axis.rotation_matrix())
            .reduce(|a, b| row_mat3_mul(a, b))
            .unwrap()
    }
}

impl From<&str> for Rotations {
    fn from(input: &str) -> Self {
        Self(input.trim().chars().map(|c| c.into()).collect())
    }
}

const ROTATIONS: [&str; 24] = [
    "X",      // right side and rotate
    "XX",     //
    "XXX",    //
    "XXXX",   // starting position, ultimately
    "Y",      // back and rotate
    "YZ",     //
    "YZZ",    //
    "YZZZ",   //
    "YY",     // left side and rotate
    "YYX",    //
    "YYXX",   //
    "YYXXX",  //
    "YYY",    // front and rotate
    "YYYZ",   //
    "YYYZZ",  //
    "YYYZZZ", //
    "Z",      // up and rotate
    "ZY",     //
    "ZYY",    //
    "ZYYY",   //
    "ZZZ",    // down and rotate
    "ZZZY",   //
    "ZZZYY",  //
    "ZZZYYY", //
];

fn all_rotations() -> Vec<Matrix3<f32>> {
    let mut out = Vec::new();

    for rotation in ROTATIONS {
        let rotations: Rotations = rotation.into();
        out.push(rotations.reduce());
    }

    out
}

fn combine(scanners: Vec<Scanner>) -> Vec<Scanner> {
    let matching = {
        let mut out = None;
        for pair in (0..scanners.len()).combinations(2) {
            let mut pairs = pair.iter();
            let a = pairs.next().unwrap().clone();
            let b = pairs.next().unwrap().clone();

            let scanner_a = scanners.get(a).unwrap();
            let scanner_b = scanners.get(b).unwrap();

            if let Some(s) = scanner_a.merge(scanner_b) {
                out = Some((a, b, s));
                break;
            }
        }
        out
    };

    if let Some((a, b, s)) = matching {
        let mut scanners: Vec<Scanner> = scanners
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != a && *i != b)
            .map(|(_, s)| s)
            .cloned()
            .collect();
        scanners.push(s);
        return scanners;
    }

    scanners
}

fn main() {
    let input = input_store::get_input(2021, 19);

    let scanners: Vec<Scanner> = input.trim().split("\n\n").map(|bl| bl.into()).collect();

    // dbg!(&scanners);

    // for v in scanners.iter().combinations(2) {
    //     let mut v = v.iter();
    //     let first = v.next().unwrap().perspectives.get(3).unwrap();

    //     let second = v.next().unwrap();

    //     for (i, relative_perspective) in second.perspectives.iter().enumerate() {
    //         let similars = first.similar(&relative_perspective);
    //         if similars.len() == 0 {
    //             continue;
    //         }

    //         for (absolute, relative, _) in similars {
    //             let delta = vec3_sub(relative, absolute);
    //             let transformed_relative = relative_perspective.transform(delta);

    //             let intersection: HashSet<_> = transformed_relative
    //                 .beacons
    //                 .intersection(&first.beacons)
    //                 .collect();

    //             if intersection.len() >= 12 {
    //                 dbg!(i, intersection);
    //                 break;
    //             }
    //         }
    //     }

    //     //println!("{:?}", first.similar(second));
    // }

    let mut scanners = scanners;

    while scanners.len() > 1 {
        scanners = combine(scanners);
        println!("{}", scanners.len());
    }

    let num_beacons = {
        let s = scanners.first().unwrap();
        s.perspectives.first().unwrap().beacons.len()
    };

    println!("part_1 => {}", num_beacons);
    println!("part_2 => {}", "not done");

    //let a = [10, 2, 0];

    // for rm in all_rotations() {
    //     let result = rotate(rm, a);
    //     println!("{:?}", result);
    // }

    //dbg!(scanners.get(0).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p1_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
