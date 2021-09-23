use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    pin::Pin,
    str::FromStr,
};

use advent::fetch;
use anyhow;
use itertools::Itertools;

fn main() {
    let input = fetch::get_input(2015, 17);

    let containers = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect_vec();

    //let containers = vec![20, 15, 10, 5, 5];

    let part1: Vec<Vec<&i32>> = containers
        .iter()
        .powerset()
        .filter(|s| s.iter().copied().sum::<i32>() == 150)
        .collect_vec();

    println!("part 1 => {}", part1.len());

    let min_length = part1.iter().map(|s| s.len()).min().unwrap();
    let part2 = part1
        .iter()
        .filter(|&ps| ps.len() == min_length)
        .collect_vec()
        .len();

    println!("part 2 => {}", part2);
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
