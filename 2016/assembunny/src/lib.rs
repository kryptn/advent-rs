use std::collections::HashMap;

use advent::{
    machine,
    machine::Apply,
    parsers::{parse_isize, ws},
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, sequence::tuple, IResult,
};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl From<char> for Register {
    fn from(r: char) -> Self {
        match r {
            'a' => Register::A,
            'b' => Register::B,
            'c' => Register::C,
            'd' => Register::D,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Register(Register),
    Value(isize),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Copy(Value, Register),
    ToggledJumpNotZero(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, Value),
    ToggledCopy(Value, Value),
    Toggle(Value),
}

impl Instruction {
    fn toggle(self) -> Self {}
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (_, inst) = parse_inst(input).unwrap();
        inst
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub instructions: Vec<Instruction>,
    pub ptr: usize,
    pub memory: HashMap<Register, isize>,
}

impl State {
    fn new() -> Self {
        let instructions = Vec::new();
        let mut memory = HashMap::new();
        memory.insert(Register::A, 0);
        memory.insert(Register::B, 0);
        memory.insert(Register::C, 0);
        memory.insert(Register::D, 0);

        Self {
            instructions,
            ptr: 0,
            memory,
        }
    }

    fn get_value(&self, value: Value) -> isize {
        match value {
            Value::Register(r) => self.memory.get(&r).unwrap().to_owned(),
            Value::Value(v) => v,
        }
    }

    pub fn memory_with(&self, register: Register, value: isize) -> HashMap<Register, isize> {
        let mut memory = self.memory.clone();
        memory.insert(register, value);
        memory
    }

    pub fn step(self) -> Option<Self> {
        let inst = self.instructions.get(self.ptr)?;
        let next_state = self.apply(inst.to_owned());
        Some(next_state)
    }
}

impl From<Vec<Instruction>> for State {
    fn from(instructions: Vec<Instruction>) -> Self {
        let empty = Self::new();

        Self {
            instructions,
            ..empty
        }
    }
}

impl From<String> for State {
    fn from(input: String) -> Self {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let (_, inst) = parse_inst(line).unwrap();
                inst
            })
            .collect();

        let empty = Self::new();

        Self {
            instructions,
            ..empty
        }
    }
}

impl machine::Apply<Instruction> for State {
    fn apply(&self, change: Instruction) -> Self {
        match change {
            Instruction::Copy(value, reg) => Self {
                instructions: self.instructions.clone(),
                memory: self.memory_with(reg, self.get_value(value)),
                ptr: self.ptr + 1,
            },
            Instruction::Increment(reg) => Self {
                instructions: self.instructions.clone(),
                memory: self.memory_with(reg, self.get_value(Value::Register(reg)) + 1),
                ptr: self.ptr + 1,
            },
            Instruction::Decrement(reg) => Self {
                instructions: self.instructions.clone(),
                memory: self.memory_with(reg, self.get_value(Value::Register(reg)) - 1),
                ptr: self.ptr + 1,
            },
            Instruction::JumpNotZero(test, offset) => Self {
                instructions: self.instructions.clone(),
                memory: self.memory.clone(),
                ptr: if self.get_value(test) != 0 {
                    (self.ptr as isize + self.get_value(offset)) as usize
                } else {
                    self.ptr + 1
                },
            },
            Instruction::Toggle(offset) => Self {
                instructions: self.instructions.clone().toggle(self.get_value(offset)),
                memory: self.memory.clone(),
                ptr: self.ptr + 1,
            },
            Instruction::ToggledCopy(value, reg) => Self {
                instructions: self.instructions.clone(),
                memory: self.memory.clone(),
                ptr: self.ptr + 1,
            },
            Instruction::ToggledJumpNotZero(_, _) => todo!(),
        }
    }
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, v) = ws(parse_isize)(input)?;
    Ok((input, Value::Value(v)))
}

fn parse_register(input: &str) -> IResult<&str, Value> {
    let (input, r) = ws(one_of("abcd"))(input)?;
    Ok((input, Value::Register(r.into())))
}

fn parse_operand(input: &str) -> IResult<&str, Value> {
    alt((parse_register, parse_value))(input)
}

fn parse_cpy(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, x, y)) = tuple((ws(tag("cpy")), parse_operand, parse_register))(input)?;
    if let Value::Register(y) = y {
        Ok((input, Instruction::Copy(x, y)))
    } else {
        unreachable!()
    }
}

fn parse_inc(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, r)) = tuple((ws(tag("inc")), parse_register))(input)?;

    if let Value::Register(r) = r {
        Ok((input, Instruction::Increment(r)))
    } else {
        unreachable!()
    }
}

fn parse_dec(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, r)) = tuple((ws(tag("dec")), parse_register))(input)?;

    if let Value::Register(r) = r {
        Ok((input, Instruction::Decrement(r)))
    } else {
        unreachable!()
    }
}

fn parse_tgl(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, v)) = tuple((ws(tag("tgl")), parse_value))(input)?;

    if let Value::Value(_) = v {
        Ok((input, Instruction::Toggle(v)))
    } else {
        unreachable!()
    }
}

fn parse_jnz(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, test, offset)) = tuple((ws(tag("jnz")), parse_operand, parse_operand))(input)?;
    Ok((input, Instruction::JumpNotZero(test, offset)))
}

fn parse_inst(input: &str) -> IResult<&str, Instruction> {
    alt((parse_cpy, parse_inc, parse_dec, parse_jnz))(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_copy_inst() {
        assert_eq!(2, 2);

        let state = State::new();
        let next_state = state.apply(Instruction::Copy(Value::Value(10), Register::A));

        assert_eq!(next_state.memory, state.memory_with(Register::A, 10));
        assert_eq!(next_state.ptr, 1);
    }

    #[rstest]
    #[case(Instruction::Copy(Value::Value(10), Register::A), 10)]
    #[case(Instruction::Increment(Register::A), 1)]
    #[case(Instruction::Decrement(Register::A), -1)]
    fn write_tests(#[case] inst: Instruction, #[case] expected_in_a: isize) {
        let state = State::new();
        let next_state = state.apply(inst);
        assert_eq!(
            next_state.memory,
            state.memory_with(Register::A, expected_in_a)
        );
        assert_eq!(next_state.ptr, 1);
    }

    #[rstest]
    #[case([0, 0, 0, 0], Instruction::JumpNotZero(Value::Register(Register::A), Value::Value(2)), 1)]
    #[case([1, 0, 0, 0], Instruction::JumpNotZero(Value::Register(Register::A), Value::Value(2)), 2)]
    #[case([2, 0, 0, 0], Instruction::JumpNotZero(Value::Register(Register::B), Value::Register(Register::A)), 1)]
    #[case([2, 1, 0, 0], Instruction::JumpNotZero(Value::Register(Register::B), Value::Register(Register::A)), 2)]
    #[trace]
    fn jnz_tests(
        #[case] starting_memory: [isize; 4],
        #[case] inst: Instruction,
        #[case] expected_offset: usize,
    ) {
        let mut memory = HashMap::new();
        memory.insert(Register::A, starting_memory[0]);
        memory.insert(Register::B, starting_memory[1]);
        memory.insert(Register::C, starting_memory[2]);
        memory.insert(Register::D, starting_memory[3]);
        let state = State {
            memory,
            ptr: 10,
            ..State::new()
        };

        let next_state = state.apply(inst);

        assert_eq!(next_state.ptr, state.ptr + expected_offset);
    }
}
