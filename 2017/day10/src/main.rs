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

    let lengths: Vec<usize> = input
        .trim()
        .as_bytes()
        .iter()
        .cloned()
        .map(|b| b.into())
        .chain(vec![17, 31, 73, 47, 23])
        .collect();
    let mut hasher = KnotHasher::new(256, lengths);
    for _ in 0..64 {
        hasher.next();
    }

    let dense_hash: Vec<usize> = hasher
        .ring
        .chunks(16)
        .map(|ch| ch.iter().cloned().reduce(|a, b| a ^ b).unwrap())
        .collect();

    let hex_string = dense_hash.iter().map(|ch| format!("{:02x}", ch)).join("");

    println!("part_2 => {}", hex_string);
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
