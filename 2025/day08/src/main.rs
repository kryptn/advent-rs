use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::{parser_helpers::just_numbers, spatial::Coordinate3d};
use itertools::Itertools;

const YEAR: usize = 2025;
const DAY: usize = 08;

const TEST_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

fn show_circuits(circuits: &Vec<HashSet<Coordinate3d>>) {
    for (idx, circuit) in circuits.iter().enumerate() {
        println!("circuit {}:", idx);
        for coord in circuit {
            println!("  {}", coord);
        }
    }
}

fn main() {
    let (input, take_amt) = {
        let input = input_store::get_input(YEAR, DAY);
        let take_amt = 1000;
        (input, take_amt)
    };

    // let (input, take_amt) = {
    //     let input = TEST_INPUT.to_string();
    //     let take_amt = 10;
    //     (input, take_amt)
    // };

    let coordinates = just_numbers::<u64>(&input)
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]).into())
        .collect::<Vec<Coordinate3d>>();

    let mut distance_map: HashMap<(Coordinate3d, Coordinate3d), f64> = HashMap::new();
    for i in 0..coordinates.len() {
        for j in 0..coordinates.len() {
            if i == j {
                continue;
            }
            let a = &coordinates[i];
            let b = &coordinates[j];
            if distance_map.contains_key(&(*a, *b)) || distance_map.contains_key(&(*b, *a)) {
                continue;
            }
            let dist = a.euclidean_distance(b);
            distance_map.insert((*a, *b), dist);
        }
    }

    let all_boxes: HashSet<Coordinate3d> = coordinates.iter().cloned().collect();
    let mut circuits: Vec<HashSet<Coordinate3d>> = vec![];

    let mut part_1 = 0;
    let mut part_2 = 0;

    distance_map
        .iter()
        .sorted_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .enumerate()
        .for_each(|(idx, ((a, b), _))| {
            let mut found_sets_idx = vec![];
            for (idx, circuit) in circuits.iter_mut().enumerate() {
                if circuit.contains(a) || circuit.contains(b) {
                    circuit.insert(*a);
                    circuit.insert(*b);
                    found_sets_idx.push(idx);
                }
            }
            if found_sets_idx.is_empty() {
                let mut new_set = HashSet::new();
                new_set.insert(*a);
                new_set.insert(*b);
                circuits.push(new_set);
            } else if found_sets_idx.len() > 1 {
                let first_idx = found_sets_idx[0];
                for &other_idx in &found_sets_idx[1..] {
                    let other_set = circuits.remove(other_idx);
                    circuits[first_idx].extend(other_set.iter());
                }
            }

            if idx + 1 == take_amt {
                part_1 = circuits
                    .iter()
                    .map(|c| c.len())
                    .sorted()
                    .rev()
                    .take(3)
                    .product::<usize>();
            }
            if circuits.len() == 1 && circuits[0] == all_boxes && part_2 == 0 {
                part_2 = a.x * b.x;
            }

            // show_circuits(&circuits);
        });

    println!("part_1 => {}", part_1);
    println!("part_2 => {}", part_2);
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
