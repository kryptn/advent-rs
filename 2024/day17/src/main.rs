use std::convert::TryFrom;

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

// impl TryFrom<&str> for Instruction {
//     type Error = &'static str;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "0" => Ok(Instruction::ADV),
//             "1" => Ok(Instruction::BXL),
//             "2" => Ok(Instruction::BST),
//             "3" => Ok(Instruction::JNZ),
//             "4" => Ok(Instruction::BXC),
//             "5" => Ok(Instruction::OUT),
//             "6" => Ok(Instruction::BDV),
//             "7" => Ok(Instruction::CDV),
//             _ => Err("Invalid instruction"),
//         }
//     }
// }

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

    desc: String,
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

    fn run(&self) -> Self {
        let mut out = self.clone();
        println!("{}", out);

        while let Some(m) = out.next() {
            println!("{}", m);

            out = m;
        }

        out
    }

    fn output_string(&self) -> String {
        self.output.iter().join(",")
    }

    // fn new_from_instructions(input: &str) -> Self {
    //     let instructions= just_numbers(input);

    //     Self {
    //         a: 0,
    //         b: 0,
    //         c: 0,
    //         pointer: 0,
    //         instructions,
    //         output: vec![],
    //     }
    // }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register A: {}\n", self.a)?;
        write!(f, "Register B: {}\n", self.b)?;
        write!(f, "Register C: {}\n", self.c)?;

        let mut padding = "   ".repeat(self.pointer + 1);
        if self.pointer > 10 {
            padding = padding[1..].to_string();
        }
        if self.pointer > 100 {
            padding = padding[1..].to_string();
        }
        write!(f, "Pointer: {}{padding} v\n", self.pointer)?;
        write!(f, "Program:     {:?}\n", self.instructions)?;
        write!(f, "last instruction desc: {}\n", self.desc)?;
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
            desc: "".to_string(),
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
                out.desc = format!("ADV: a = {} / 2^{}", self.a, out.combo(out.pointer + 1));
            }
            Instruction::BXL => {
                // bitwise xor
                out.b = out.b ^ out.literal(out.pointer + 1);
                out.desc = format!("BXL: b = {} ^ {}", self.b, out.literal(out.pointer + 1));
            }
            Instruction::BST => {
                // mod 8
                out.b = out.combo(out.pointer + 1) % 8;
                out.desc = format!("BST: b = {} % 8", self.b);
            }
            Instruction::JNZ => {
                if out.a != 0 {
                    out.pointer = out.literal(out.pointer + 1);
                    jumped = true;
                    out.desc = format!("JNZ: a != 0, jump to {}", out.pointer);
                } else {
                    out.desc = format!("JNZ: a == 0");
                }
            }
            Instruction::BXC => {
                // bitwise xor b c
                out.b = out.b ^ out.c;
                out.desc = format!("BXC: b = {} ^ {}", self.b, self.c);
            }
            Instruction::OUT => {
                let value = out.combo(out.pointer + 1) % 8;
                out.output.push(value as u8);
                out.desc = format!("OUT: output {}", value);
            }
            Instruction::BDV => {
                let denominator = 2usize.pow(out.combo(out.pointer + 1) as u32);
                out.b = (out.a as f64 / denominator as f64).trunc() as usize;
                out.desc = format!("BDV: b = {} / 2^{}", self.a, out.combo(out.pointer + 1));
            }
            Instruction::CDV => {
                let denominator = 2usize.pow(out.combo(out.pointer + 1) as u32);
                out.c = (out.a as f64 / denominator as f64).trunc() as usize;
                out.desc = format!("CDV: c = {} / 2^{}", self.a, out.combo(out.pointer + 1));
            }
        }

        if !jumped {
            out.pointer += 2;
        }

        Some(out)
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let machine = Machine::from(input.as_str());

    let mut running_machine = machine.clone();
    // println!("{}", running_machine);
    while let Some(m) = running_machine.next() {
        // println!("{}", m);
        // std::thread::sleep(std::time::Duration::from_millis(100));
        running_machine = m;
    }

    println!("{}", running_machine);

    println!("part_1 => {}", running_machine.output_string());
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
