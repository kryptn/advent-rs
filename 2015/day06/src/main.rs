use advent::{
    fetch,
    grid::{coordinate_str, coordinates_within, Coordinate, Grid},
};
use anyhow;
use std::{cmp::max, str::FromStr};

#[derive(Debug, PartialEq)]
struct Selection {
    a: Coordinate,
    b: Coordinate,
}

#[derive(Debug, PartialEq)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    selection: Selection,
    action: Action,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();
        if tokens[0] == "turn" {
            let a = coordinate_str(tokens[2], ",");
            let b = coordinate_str(tokens[4], ",");
            let selection = Selection { a, b };

            if tokens[1] == "on" {
                Ok(Instruction {
                    selection,
                    action: Action::On,
                })
            } else {
                Ok(Instruction {
                    selection,
                    action: Action::Off,
                })
            }
        } else {
            let a = coordinate_str(tokens[1], ",");
            let b = coordinate_str(tokens[3], ",");
            let selection = Selection { a, b };
            Ok(Instruction {
                selection,
                action: Action::Toggle,
            })
        }
    }
}

fn part1(input: String) -> usize {
    let mut wall = Grid::<bool>::new();
    for dot in coordinates_within(Coordinate::new(0, 0), Coordinate::new(999, 999)) {
        wall.insert(dot, false);
    }

    for instruction in input.lines().map(|l| Instruction::from_str(l).unwrap()) {
        for coordinate in coordinates_within(instruction.selection.a, instruction.selection.b) {
            match instruction.action {
                Action::On => *wall.get_mut(&coordinate).unwrap() = true,
                Action::Off => *wall.get_mut(&coordinate).unwrap() = false,
                Action::Toggle => *wall.get_mut(&coordinate).unwrap() = !wall[&coordinate],
            }
        }
    }

    let items: Vec<&bool> = wall.values().filter(|v| **v).collect();

    items.len()
}

fn part2(input: String) -> i32 {
    let mut wall = Grid::<i32>::new();
    for dot in coordinates_within(Coordinate::new(0, 0), Coordinate::new(999, 999)) {
        wall.insert(dot, 0);
    }

    for instruction in input.lines().map(|l| Instruction::from_str(l).unwrap()) {
        for coordinate in coordinates_within(instruction.selection.a, instruction.selection.b) {
            match instruction.action {
                Action::On => *wall.get_mut(&coordinate).unwrap() += 1,
                Action::Off => *wall.get_mut(&coordinate).unwrap() = max(0, wall[&coordinate] - 1),
                Action::Toggle => *wall.get_mut(&coordinate).unwrap() += 2,
            }
        }
    }

    wall.values().sum()
}

fn main() {
    let input = fetch::get_input(2015, 6);

    println!("part 1 = {}", part1(input.clone()));
    println!("part 2 = {}", part2(input.clone()));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn check_instruction() {
        let value = "turn on 0,0 through 999,999";
        let action = Instruction::from_str(value).unwrap();

        assert_eq!(
            action,
            Instruction {
                selection: Selection {
                    a: Coordinate::new(0, 0),
                    b: Coordinate::new(999, 999)
                },
                action: Action::On
            }
        );
    }
}
