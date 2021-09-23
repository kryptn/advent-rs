use std::{collections::HashMap, slice::Iter};

use advent::{fetch, numbers::factors};
use itertools::{Combinations, Itertools, Powerset};

fn part_1(input: i32) -> i32 {
    let mut houses: HashMap<i32, i32> = HashMap::new();

    for i in 1..(input / 10) {
        for j in (i..(input / 10)).step_by(i as usize) {
            if houses.contains_key(&j) {
                *houses.get_mut(&j).unwrap() += i * 10;
            } else {
                houses.insert(j, i * 10);
            }
        }
    }

    let mut lowest = input;
    for (house, presents) in houses {
        if presents >= input && house < lowest {
            lowest = house;
        }
    }

    lowest
}

fn part_2(input: i32) -> i32 {
    let mut houses: HashMap<i32, i32> = HashMap::new();

    for i in 1..(input / 10) {
        for (h, j) in (i..(input / 10)).step_by(i as usize).enumerate() {
            if h >= 50 {
                break;
            }
            if houses.contains_key(&j) {
                *houses.get_mut(&j).unwrap() += i * 11;
            } else {
                houses.insert(j, i * 11);
            }
        }
    }

    let mut lowest = input;
    for (house, presents) in houses {
        if presents >= input && house < lowest {
            lowest = house;
        }
    }

    lowest
}

fn main() {
    let input = fetch::get_input(2015, 20);
    let input = input.trim().parse::<i32>().unwrap();

    println!("part 1 => {}", part_1(input));

    println!("part 2 => {}", part_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    // #[test]
    // fn ps_test() {
    //     let mut f = Factors::new();

    // }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
