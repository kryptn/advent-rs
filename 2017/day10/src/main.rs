use std::collections::{HashMap, HashSet};

use advent::input_store;

fn step(position: usize, length: usize, ring: &mut Vec<usize>) {
    // dbg!(position, length, &ring);
    ring.rotate_left(position);

    let left = &mut ring[0..length];
    left.reverse();

    ring.rotate_right(position);
}

fn main() {
    let input = input_store::get_input(2017, 10);
    // let input = "3,4,1,5";
    let lengths: Vec<usize> = input
        .trim()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();

    let mut ring: Vec<usize> = (0..256).into_iter().collect();

    let mut position = 0;
    let mut skip = 0;
    for length in lengths {
        step(position, length, &mut ring);
        position = (position + length + skip) % ring.len();
        skip += 1;
    }

    println!("part_1 => {}", ring[0] * ring[1]);
    println!("part_2 => {}", "not done");
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
