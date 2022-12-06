use std::collections::HashMap;

use advent::input_store;
use itertools::Itertools;

#[derive(Clone)]
struct Stacks(HashMap<usize, Vec<char>>);

impl From<&str> for Stacks {
    fn from(input: &str) -> Self {
        let raw_stacks: Vec<_> = input.lines().rev().collect();

        let stacks: HashMap<usize, Vec<char>> = {
            let mut out: HashMap<_, Vec<char>> = HashMap::new();
            let mut raw = raw_stacks.iter();
            let target_idx: Vec<(usize, usize)> = raw
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| ('0'..='9').contains(c))
                .map(|(i, c)| (i.clone(), c.to_string().parse::<usize>().unwrap()))
                .collect();

            for line in raw {
                let chars: Vec<char> = line.chars().collect();
                for (target, stack) in target_idx.clone() {
                    let candidate = chars[target];
                    if candidate != ' ' {
                        out.entry(stack).or_default().push(candidate)
                    }
                }
            }
            out
        };

        Self(stacks)
    }
}

impl Stacks {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            let temp = self.0.entry(instruction.from).or_default().pop().unwrap();
            self.0.entry(instruction.to).or_default().push(temp);
        }
    }

    fn apply_instruction_5001(&mut self, instruction: &Instruction) {
        let mut stack = Vec::new();
        for _ in 0..instruction.quantity {
            let temp = self.0.entry(instruction.from).or_default().pop().unwrap();
            stack.push(temp);
        }

        for item in stack.clone().iter().rev() {
            self.0.entry(instruction.to).or_default().push(*item)
        }
    }
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        let quantity = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        Self { quantity, from, to }
    }
}

fn main() {
    let input = input_store::get_input(2022, 05);
    let parts: Vec<_> = input.split("\n\n").collect();

    let stacks = Stacks::from(parts[0]);
    let instructions: Vec<Instruction> = parts[1].trim().lines().map(|line| line.into()).collect();

    let mut p1_stacks = stacks.clone();
    for instruction in instructions.iter() {
        p1_stacks.apply_instruction(instruction)
    }
    let part_1: String = p1_stacks
        .0
        .iter()
        .sorted()
        .map(|(_, v)| v.last().unwrap())
        .collect();

    println!("part_1 => {}", part_1);

    let mut p2_stacks = stacks.clone();
    for instruction in instructions.iter() {
        p2_stacks.apply_instruction_5001(&instruction)
    }
    let part_2: String = p2_stacks
        .0
        .iter()
        .sorted()
        .map(|(_, v)| v.last().unwrap())
        .collect();
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
