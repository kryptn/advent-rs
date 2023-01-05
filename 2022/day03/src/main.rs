use std::collections::{HashMap, HashSet};

use advent::input_store;
use itertools::Itertools;

fn main() {
    let priorities: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let input = input_store::get_input(2022, 03);

    let part_1: usize = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left: HashSet<char> = left.chars().collect();
            let right: HashSet<char> = right.chars().collect();
            let common = left.intersection(&right).into_iter().next().unwrap();
            priorities[common]
        })
        .sum();

    let part_2: usize = input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|ch| {
            let chunks: Vec<HashSet<char>> = ch
                .map(|line| {
                    let chunk: HashSet<char> = line.trim().chars().collect();
                    chunk
                })
                .collect();

            let a: HashSet<_> = chunks[0].intersection(&chunks[1]).cloned().collect();
            let b: HashSet<_> = a.intersection(&chunks[2]).collect();

            let common = b.into_iter().next().unwrap();
            priorities[common]
        })
        .sum();

    println!("part_1 => {}", part_1);
    println!("part_2 => {}", part_2);
}

#[cfg(test)]
mod test {

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
