use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::hashers::knot::KnotHasher;
use itertools::Itertools;


fn main() {
    let input = input_store::get_input(2017, 10);
    // let input = "3,4,1,5";
    let lengths: Vec<usize> = input
        .trim()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();

    let mut hasher = KnotHasher::new(256, lengths);
    let hashed = hasher.next().unwrap();

    println!("part_1 => {}", hashed[0] * hashed[1]);

    let mut hasher = KnotHasher::new_from_str(256, &input);
    hasher.round();

    println!("part_2 => {}", hasher.as_hex_str());
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
