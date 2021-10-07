use std::{collections::HashMap, str::FromStr};

use advent::{input_store, machine};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Debug)]
enum Value {
    Register(Register),
    Value(isize),
}

#[derive(Clone, Debug)]
enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, Value),
}

#[derive(Clone, Debug)]
struct State {
    instructions: Vec<Instruction>,
    ptr: usize,
    memory: HashMap<Register, isize>,
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

    fn memory_with(&self, register: Register, value: isize) -> HashMap<Register, isize> {
        let mut memory = self.memory.clone();
        memory.insert(register, value);
        memory
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
                    //self.ptr + self.get_value(offset) as usize
                    (self.ptr as isize + self.get_value(offset)) as usize
                } else {
                    self.ptr + 1
                },
            },
        }
    }
}

fn main() {
    let input = input_store::get_input(2016, 12);
}

#[cfg(test)]
mod test {
    use super::*;
    use advent::machine::Apply;
    use rstest::*;

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
