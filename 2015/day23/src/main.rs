use std::{collections::HashMap, ops::Index};

use advent::fetch;
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take},
    character::complete::{char, digit1, newline, one_of, space0},
    combinator::opt,
    multi::many0,
    IResult,
};

#[derive(Debug, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Register {
    A,
    B,
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    let (input, reg) = take(1 as usize)(input)?;

    if reg == "a" {
        Ok((input, Register::A))
    } else {
        Ok((input, Register::B))
    }
}

fn parse_offset(input: &str) -> IResult<&str, i32> {
    let (input, p) = one_of("+-")(input)?;
    let (input, n) = digit1(input)?;

    let mut n = n.parse::<i32>().unwrap();
    if p == '-' {
        n *= -1;
    }
    Ok((input, n))
}

fn parse_hlf(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("hlf")(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = parse_register(input)?;

    Ok((input, Instruction::Half(register)))
}
fn parse_tpl(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("tpl")(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = parse_register(input)?;
    Ok((input, Instruction::Triple(register)))
}
fn parse_inc(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("inc")(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = parse_register(input)?;
    Ok((input, Instruction::Increment(register)))
}
fn parse_jmp(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("jmp")(input)?;
    let (input, _) = space0(input)?;
    let (input, offset) = parse_offset(input)?;
    Ok((input, Instruction::Jump(offset)))
}
fn parse_jie(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("jie")(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = parse_register(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;
    let (input, offset) = parse_offset(input)?;
    Ok((input, Instruction::JumpIfEven(register, offset)))
}
fn parse_jio(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag_no_case("jio")(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = parse_register(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;
    let (input, offset) = parse_offset(input)?;
    Ok((input, Instruction::JumpIfOne(register, offset)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, inst) = alt((
        parse_hlf, parse_tpl, parse_inc, parse_jmp, parse_jie, parse_jio,
    ))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((input, inst))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

#[derive(Debug, Clone)]
struct Machine {
    instructions: Vec<Instruction>,
    memory: HashMap<Register, i32>,
    ptr: i32,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut memory = HashMap::new();
        memory.insert(Register::A, 0);
        memory.insert(Register::B, 0);

        Self {
            instructions,
            memory,
            ptr: 0,
        }
    }

    fn new_with_state(instructions: Vec<Instruction>, a: i32, b: i32) -> Self {
        let mut memory = HashMap::new();
        memory.insert(Register::A, a);
        memory.insert(Register::B, b);

        Self {
            instructions,
            memory,
            ptr: 0,
        }
    }

    fn step(&self) -> Option<Self> {
        if self.ptr < 0 || self.ptr as usize >= self.instructions.len() {
            return None;
        }

        let mut new_machine = self.clone();

        let instruction = self.instructions.index(self.ptr as usize).to_owned();

        match instruction {
            Instruction::Half(reg) => {
                new_machine.memory.insert(reg, self.memory[&reg] / 2);
                new_machine.ptr += 1;
            }
            Instruction::Triple(reg) => {
                new_machine.memory.insert(reg, self.memory[&reg] * 3);
                new_machine.ptr += 1;
            }
            Instruction::Increment(reg) => {
                new_machine.memory.insert(reg, self.memory[&reg] + 1);
                new_machine.ptr += 1;
            }
            Instruction::Jump(offset) => new_machine.ptr += offset,
            Instruction::JumpIfEven(reg, offset) => {
                if self.memory[&reg] % 2 == 0 {
                    new_machine.ptr += offset;
                } else {
                    new_machine.ptr += 1;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if self.memory[&reg] == 1 {
                    new_machine.ptr += offset;
                } else {
                    new_machine.ptr += 1;
                }
            }
        };

        Some(new_machine)
    }

    fn mut_step(&mut self) -> Option<()> {
        match self.step() {
            Some(next) => {
                self.memory = next.memory;
                self.ptr = next.ptr;
                Some(())
            }
            None => None,
        }
    }
}

impl Iterator for Machine {
    type Item = HashMap<Register, i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mut_step() {
            Some(_) => Some(self.memory.clone()),
            None => {
                println!("final state => {:?}", self.memory.clone());
                None
            }
        }
    }
}

fn main() {
    let input = fetch::get_input(2015, 23);
    let (_, instructions) = parse_instructions(&input).unwrap();

    let machine = Machine::new(instructions.clone());
    for _ in machine {}

    let machine = Machine::new_with_state(instructions, 1, 0);
    for _ in machine {}
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
