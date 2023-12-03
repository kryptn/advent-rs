use std::collections::{HashMap, HashSet};

use colored::Colorize;

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};

const YEAR: usize = 2023;
const DAY: usize = 3;

#[derive(Default, Clone, Debug)]
struct Part {
    value: char,
    valid: bool,
}

impl Part {
    fn is_number(&self) -> bool {
        "0123456789".contains(self.value)
    }
    fn is_symbol(&self) -> bool {
        !" .0123456789".contains(self.value)
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.valid {
            write!(f, "{}", self.value.to_string().green())
        } else {
            if self.is_number() {
                write!(f, "{}", self.value.to_string().red())
            } else {
                write!(f, "{}", self.value)
            }
        }
    }
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        Self {
            value,
            valid: false,
        }
    }
}

fn convert_number(digits: Vec<Coordinate>, engine: &Space<Coordinate, Part>) -> i32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let d = engine
                .get(c)
                .unwrap()
                .value
                .to_string()
                .parse::<i32>()
                .unwrap();
            d * 10_i32.pow(idx as u32)
        })
        .sum()
}

fn extract_numbers(space: &Space<Coordinate, Part>) -> Vec<(i32, Vec<Coordinate>)> {
    let mut out = vec![];

    let mut numbers = vec![];

    for row in space.rows() {
        for (coord, part) in row {
            if part.is_number() {
                numbers.push(coord);
            } else {
                if numbers.len() > 0 {
                    let number = convert_number(numbers.clone(), space);
                    out.push((number, numbers.clone()));
                    numbers = vec![];
                }
            }
        }

        if numbers.len() > 0 {
            let number = convert_number(numbers.clone(), space);
            out.push((number, numbers.clone()));
            numbers = vec![];
        }
    }

    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let input = input.as_str();

    // let input = r#"467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    // "#;

    // let input = "123@456";

    let mut engine: Space<Coordinate, Part> = Space::from(input);

    let numbers = extract_numbers(&engine);
    // dbg!(&numbers);

    let valid_numbers: Vec<(i32, Vec<Coordinate>)> = numbers
        .iter()
        .filter_map(|(number, coords)| {
            let surrounding_coords = coords
                .iter()
                .map(|c| c.neighbors())
                .flatten()
                .filter(|c| engine.contains_key(c))
                .collect::<HashSet<_>>();

            if surrounding_coords
                .iter()
                .any(|c| engine.get(c).unwrap().is_symbol())
            {
                for c in coords {
                    engine.get_mut(c).unwrap().valid = true;
                }

                Some((*number, surrounding_coords.into_iter().collect()))
            } else {
                None
            }
        })
        .collect();

    let valid_sum = valid_numbers.iter().map(|(n, _)| n).sum::<i32>();
    // println!("engine:\n\n{}", engine);

    println!("part_1 => {}", valid_sum);

    let mut gears: HashMap<Coordinate, Vec<i32>> = engine
        .iter()
        .filter(|(_, part)| part.value == '*')
        .map(|(coord, _)| (*coord, Vec::new()))
        .collect();

    for (number, coords) in valid_numbers {
        for coord in coords {
            if gears.contains_key(&coord) {
                gears.get_mut(&coord).unwrap().push(number);
            }
        }
    }

    let ratio_sum = gears
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum::<i32>();

    // dbg!(gears);

    println!("part_2 => {}", ratio_sum);
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
