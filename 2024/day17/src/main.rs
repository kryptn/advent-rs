use std::{convert::TryFrom, usize::MAX};

use advent::input_store;
use advent_toolbox::parser_helpers::just_numbers;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 17;

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<u8> for Instruction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::ADV),
            1 => Ok(Instruction::BXL),
            2 => Ok(Instruction::BST),
            3 => Ok(Instruction::JNZ),
            4 => Ok(Instruction::BXC),
            5 => Ok(Instruction::OUT),
            6 => Ok(Instruction::BDV),
            7 => Ok(Instruction::CDV),
            _ => Err("Invalid instruction"),
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,

    pointer: usize,
    instructions: Vec<u8>,

    output: Vec<u8>,
}

impl Machine {
    fn literal(&mut self, at: usize) -> usize {
        self.instructions.get(at).unwrap().clone() as usize
    }

    fn combo(&mut self, at: usize) -> usize {
        let value = self.instructions.get(at).unwrap().clone();
        match value {
            0..=3 => value as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("unused in valid programs lol sure"),
            _ => panic!("Invalid operand"),
        }
    }

    fn instruction(&self, at: usize) -> Instruction {
        let raw = self.instructions.get(at).unwrap();
        let instruction = Instruction::try_from(*raw).unwrap();
        instruction
    }

    fn output_string(&self) -> String {
        self.output.iter().join(",")
    }

    fn with_octets(&self, octets: Vec<u8>) -> Self {
        let a = octets_as_usize(&octets);

        Self { a, ..self.clone() }
    }

    fn run_output(&self) -> Vec<u8> {
        let mut out = self.clone();
        while let Some(m) = out.next() {
            out = m;
        }

        out.output
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register A: {}\n", self.a)?;
        write!(f, "Register B: {}\n", self.b)?;
        write!(f, "Register C: {}\n", self.c)?;

        let mut padding = "   ".repeat(self.pointer + 1);
        if self.pointer >= 10 {
            padding = padding[1..].to_string();
        }
        if self.pointer >= 100 {
            padding = padding[1..].to_string();
        }
        write!(f, "Pointer: {}{padding} v\n", self.pointer)?;
        write!(f, "Program:     {:?}\n", self.instructions)?;
        write!(f, "Output: {:?}\n\n", self.output)
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let mut values = just_numbers(input).into_iter();

        let a = values.next().unwrap();
        let b = values.next().unwrap();
        let c = values.next().unwrap();

        let instructions = values.map(|n| n as u8).collect();

        Self {
            a,
            b,
            c,
            pointer: 0,
            instructions,
            output: vec![],
        }
    }
}

impl Iterator for Machine {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.instructions.len() {
            return None;
        }

        let mut out = self.clone();
        let mut jumped = false;
        match out.instruction(out.pointer) {
            Instruction::ADV => {
                let denominator = 2usize.pow(out.combo(out.pointer + 1) as u32);

                out.a = (out.a as f64 / denominator as f64).trunc() as usize;
            }
            Instruction::BXL => {
                // bitwise xor

                out.b = out.b ^ out.literal(out.pointer + 1);
            }
            Instruction::BST => {
                // mod 8
                out.b = out.combo(out.pointer + 1) % 8;
            }
            Instruction::JNZ => {
                if out.a != 0 {
                    out.pointer = out.literal(out.pointer + 1);
                    jumped = true;
                }
            }
            Instruction::BXC => {
                // bitwise xor b c
                out.b = out.b ^ out.c;
            }
            Instruction::OUT => {
                let value = out.combo(out.pointer + 1) % 8;
                out.output.push(value as u8);
            }
            Instruction::BDV => {
                let denominator = 2usize.pow(out.combo(out.pointer + 1) as u32);
                out.b = (out.a as f64 / denominator as f64).trunc() as usize;
            }
            Instruction::CDV => {
                let denominator = 2usize.pow(out.combo(out.pointer + 1) as u32);
                out.c = (out.a as f64 / denominator as f64).trunc() as usize;
            }
        }

        if !jumped {
            out.pointer += 2;
        }

        Some(out)
    }
}

fn octets_as_usize(octets: &Vec<u8>) -> usize {
    let mut a = 0;
    for octet in octets {
        a = a << 3 | *octet as usize;
    }
    a
}

fn find(machine: &Machine, a: Vec<u8>, expected: &Vec<u8>) -> Vec<(Machine, Vec<u8>)> {
    let initial = machine.with_octets(a.clone());
    let initial_output = initial.run_output();
    // print_details(&a, &initial_output, machine);
    if initial_output == *expected {
        return vec![(initial, a)];
    } else if initial_output.len() > expected.len() {
        return vec![];
    }

    if expected[expected.len() - a.len()] != initial_output[initial_output.len() - a.len()] {
        return vec![];
    }

    let mut out = vec![];

    for x in 0..8 {
        let mut next = a.clone();
        next.push(x as u8);
        out.extend(find(machine, next, expected));
    }
    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let machine = Machine::from(input.as_str());

    let mut running_machine = machine.clone();
    while let Some(m) = running_machine.next() {
        running_machine = m;
    }

    println!("part_1 => {}", running_machine.output_string());
    let expected = machine.instructions.clone();

    let (mut part_2, mut octets) = (0..8)
        .into_iter()
        .flat_map(|x| find(&machine, vec![x], &expected))
        .min_by(|a, b| a.0.a.cmp(&b.0.a))
        .unwrap();

    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..8 {
            let mut test = octets.clone();
            test[i] -= 1;
            let test_machine = machine.with_octets(test.clone());
            if test_machine.run_output() == expected && test_machine.a < part_2.a {
                changed = true;
                part_2 = test_machine;
                octets = test;
            }
        }
    }

    println!("part_2 => {:?}", part_2.a);
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
    #[case("0 0 9 2,6", |m: &Machine| m.b == 1)]
    #[case("10 0 0 5,0,5,1,5,4", |m: &Machine| m.output_string() == "0,1,2")]
    #[case("2024 0 0 0,1,5,4,3,0", |m: &Machine| m.output_string() == "4,2,5,6,7,7,7,7,3,1,0" && m.a == 0)]
    #[case("0 29 0 1,7", |m: &Machine| m.b == 26)]
    #[case("0 2024 43690 4,0", |m: &Machine| m.b == 44354)]
    #[case("729 0 0 0,1,5,4,3,0",  |m: &Machine| m.output_string() == "4,6,3,5,6,3,5,2,1,0")]
    fn p1_tests(#[case] given: &str, #[case] should_pass: impl Fn(&Machine) -> bool) {
        let machine = Machine::from(given);
        let completed = machine.run();
        println!("{}", completed);

        assert!(should_pass(&completed));
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
