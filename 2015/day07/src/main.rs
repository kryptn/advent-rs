use std::{collections::{HashMap, VecDeque}, str::FromStr};

use advent::fetch;
use anyhow;
#[derive(Debug)]

enum Value {
    Signal(u16),
    Wire(String),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(signal) => Ok(Value::Signal(signal)),
            Err(_) => Ok(Value::Wire(s.to_string())),
        }
    }
}
#[derive(Debug)]

enum Operator {
    And,
    Or,
    Not,
    LeftShift,
    RightShift,
    Provide,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operator::And),
            "OR" => Ok(Operator::Or),
            "NOT" => Ok(Operator::Not),
            "LSHIFT" => Ok(Operator::LeftShift),
            "RSHIFT" => Ok(Operator::RightShift),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]

struct Instruction {
    destination: String,
    operator: Operator,
    b: Value,
    a: Option<Value>,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inst: Vec<&str> = s.split(" ").into_iter().collect();

        let destination = inst.pop().unwrap().to_string(); // should always be a wire
        inst.pop().unwrap(); // ->
        let b = Value::from_str(inst.pop().unwrap()).unwrap();

        let operator = match inst.pop() {
            Some(val) => Operator::from_str(val).unwrap(),
            None => Operator::Provide,
        };

        let a = match inst.pop() {
            Some(val) => Some(Value::from_str(val).unwrap()),
            None => None,
        };

        Ok(Self {
            destination,
            operator,
            b,
            a,
        })
    }
}

#[derive(Debug)]
struct Machine {
    pub memory: HashMap<String, u16>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn value_of(&self, value: Value) -> u16 {
        match value {
            Value::Signal(s) => s,
            Value::Wire(w) => {
                if !self.memory.contains_key(&w) {
                    0
                } else {
                    self.memory.get(&w).unwrap().to_owned()
                }

            },
        }
    }

    pub fn can_apply(&self, inst: &Instruction) -> bool {
        if let Value::Wire(w) = &inst.b {
            if !self.memory.contains_key(w) {
                return false
            }
        };

        if let Some(Value::Wire(w)) = &inst.a {
            if !self.memory.contains_key(w) {
                return false
            }
        };

        true
    }

    pub fn step(&mut self, inst: Instruction) {

        match inst.operator {
            Operator::And => {
                self.memory.insert(inst.destination, self.value_of(inst.a.unwrap()) & self.value_of(inst.b));
            }
            Operator::Or => {
                self.memory.insert(inst.destination, self.value_of(inst.a.unwrap()) | self.value_of(inst.b));
            }
            Operator::Not => {
                self.memory.insert(inst.destination, !self.value_of(inst.b));
            }
            Operator::LeftShift => {
                self.memory.insert(inst.destination, self.value_of(inst.a.unwrap()) << self.value_of(inst.b));
            }
            Operator::RightShift => {
                self.memory.insert(inst.destination, self.value_of(inst.a.unwrap()) >> self.value_of(inst.b));
            }
            Operator::Provide => {
                self.memory.insert(inst.destination, self.value_of(inst.b));
            }
        }
    }
}

fn main() {
    let input = fetch::get_input(2015, 7);
    // let input = r#"123 -> x
    // 456 -> y
    // x AND y -> d
    // x OR y -> e
    // x LSHIFT 2 -> f
    // y RSHIFT 2 -> g
    // NOT x -> h
    // NOT y -> i"#;

    let mut machine = Machine::new();

    let mut instructions: VecDeque<Instruction> = input.lines().map(|i| {
        let trimmed = i.trim();
        let mut i = Instruction::from_str(trimmed).unwrap();

        // part 2
        if i.destination == String::from("b") {
            i.b = Value::Signal(956);
        }
        
        i
    } ).collect();

    while instructions.len() > 0 {
        if machine.can_apply(&instructions[0]) {
            let inst = instructions.pop_front().expect("this should exist");
            machine.step(inst);
        }
        if instructions.len() > 1 {
            instructions.rotate_left(1);
        }

    }

    println!("part 1 => {}", machine.memory.get(&String::from("a")).unwrap());

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

    }
}
