use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 08;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............
    // "#;

    let space: Space<Coordinate, char> = input.into();

    let mut stations: HashMap<char, HashSet<Coordinate>> = HashMap::new();

    for (coord, tile) in space.iter() {
        match tile {
            '.' => {}
            c => {
                let entry = stations.entry(*c).or_insert(HashSet::new());
                entry.insert(*coord);
            }
        }
    }

    let mut part_1 = HashSet::new();

    for coords in stations.values() {
        for p in coords.iter().permutations(2) {
            let (a, b) = (p[0], p[1]);
            let dist = *b - *a;

            part_1.insert(*b + dist);
        }
    }

    let part_1 = part_1.iter().filter(|c| space.contains_key(c)).count();

    println!("part_1 => {}", part_1);

    let mut part_2 = HashSet::new();

    for coords in stations.values() {
        for p in coords.iter().permutations(2) {
            let (a, b) = (p[0], p[1]);
            let dist = *b - *a;
            part_2.insert(*b);
            let mut antinode = *b + dist;
            while space.contains_key(&antinode) {
                part_2.insert(antinode);
                antinode += dist;
            }
        }
    }

    let part_2 = part_2.iter().filter(|c| space.contains_key(c)).count();

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
