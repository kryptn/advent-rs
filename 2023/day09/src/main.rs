use std::{
    collections::{HashMap, HashSet},
    vec,
};

use advent::input_store;

const YEAR: usize = 2023;
const DAY: usize = 9;

fn deltas(items: &Vec<isize>) -> Vec<isize> {
    let mut out = Vec::new();
    for i in 0..items.len() - 1 {
        out.push(items[i + 1] - items[i]);
    }
    out
}

fn is_base_velocity(items: &Vec<isize>) -> bool {
    items.iter().collect::<HashSet<_>>().len() == 1
}

fn find_next_item(items: Vec<isize>) -> isize {
    let mut sequences = vec![items];
    while !is_base_velocity(sequences.last().unwrap()) {
        sequences.push(deltas(sequences.last().unwrap()));
    }
    for idx in (0..(sequences.len() - 1)).rev() {
        let lower_last = sequences[idx + 1].last().unwrap().clone();
        let this_last = sequences[idx].last().unwrap().clone();
        sequences[idx].push(lower_last + this_last);
    }

    sequences.first().unwrap().last().unwrap().clone()
}

fn parse_sequence(line: &str) -> Vec<isize> {
    line.trim()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let sequences: Vec<Vec<isize>> = input.trim().lines().map(parse_sequence).collect();

    let part_1 = sequences
        .iter()
        .map(|s| find_next_item(s.clone()))
        .sum::<isize>();
    println!("part_1 => {}", part_1);

    let part_2 = sequences
        .iter()
        .map(|s| find_next_item(s.iter().rev().cloned().collect::<Vec<_>>().clone()))
        .sum::<isize>();
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
    #[case("0   3   6   9  12  15", 18)]
    #[case("1   3   6  10  15  21", 28)]
    fn p1_tests(#[case] given: &str, #[case] expected: isize) {
        let seq = parse_sequence(given);
        let last = find_next_item(seq);
        assert_eq!(last, expected);
    }

    #[rstest]
    #[case("10  13  16  21  30  45", 5)]
    fn p2_tests(#[case] given: &str, #[case] expected: isize) {
        let seq = parse_sequence(given);
        let prev = find_next_item(seq.iter().rev().cloned().collect::<Vec<_>>());
        assert_eq!(prev, expected);
    }
}
