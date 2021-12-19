use std::collections::{HashMap, HashSet};

use advent::input_store;
use itertools::Itertools;
use quaternion::rotate_vector;
use vecmath::{vec3_sub, Vector3};

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Vector3<isize>>,
    other_beacons: Vec<Vector3<isize>>,
    distance_map: HashMap<Vector3<isize>, Vec<isize>>,
}

impl From<&str> for Scanner {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next();
        let beacons: Vec<Vector3<isize>> = lines
            .map(|l| {
                let val: Vec<isize> = l
                    .trim()
                    .split(",")
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                [val[0usize], val[1usize], val[2usize]]
            })
            .collect();

        let mut distance_map = HashMap::new();
        for this in &beacons {
            let mut distances = Vec::new();
            for other in &beacons {
                let mag = vec3_sub(*this, *other);
                let dist = mag[0].abs() + mag[1].abs() + mag[2].abs();
                distances.push(dist)
            }
            distances.sort();

            distance_map.insert(*this, distances);
        }

        Self {
            beacons,
            other_beacons: Vec::new(),
            distance_map,
        }
    }
}

impl Scanner {
    fn similar(&self, other: &Self) -> Vec<(Vector3<isize>, Vector3<isize>, usize)> {
        let mut results = Vec::new();
        for (source, source_dmap) in &self.distance_map {
            for (test, test_dmap) in &other.distance_map {
                let matched = source_dmap
                    .iter()
                    .zip(test_dmap.iter())
                    .filter(|(&a, &b)| a == b)
                    .count();
                if matched > 2 {
                    results.push((source.clone(), test.clone(), matched));
                }
            }
        }

        results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        results.reverse();

        results
    }
}

fn main() {
    let input = input_store::get_input(2021, 19);

    let scanners: Vec<Scanner> = input.trim().split("\n\n").map(|bl| bl.into()).collect();

    // dbg!(&scanners);

    println!("part_1 => {}", "not done");
    println!("part_2 => {}", "not done");

    for v in scanners.iter().combinations(2) {
        let mut v = v.iter();
        let first = v.next().unwrap();
        let second = v.next().unwrap();
        println!("{:?}", first.similar(second));
    }

    // let a = [100., 100., 100.];
    // let b = rotate_vector((0., [0., -1., 0.]), a);

    // dbg!(a, b);
    // for k in -1..2 {
    //     for j in -1..2 {
    //         for i in -1..2 {
    //             let i = i as f32;
    //             let j = j as f32;
    //             let k = k as f32;
    //             let q = (0., [i, j, k]);
    //             let b = rotate_vector(q, a);
    //             dbg!(q, b);
    //         }
    //     }
    // }
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
