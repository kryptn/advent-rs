use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::Duration;

use advent::parsers::parse_isize;
use advent::{input_store, parsers::ws};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::one_of;
use nom::IResult;

use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::W => write!(f, "w"),
            Register::X => write!(f, "x"),
            Register::Y => write!(f, "y"),
            Register::Z => write!(f, "z"),
        }
    }
}

impl From<char> for Register {
    fn from(input: char) -> Self {
        match input {
            'w' => Self::W,
            'x' => Self::X,
            'y' => Self::Y,
            'z' => Self::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Literal(isize),
    Register(Register),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(v) => write!(f, "{}", v),
            Operand::Register(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Div(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
    //Set(Register, Operand),
    Setup(Operand, Operand),
    Maama(Register, Register, Operand, Operand, Operand),
    Mamam(Register, Register, Operand, Operand, Operand),
    Oper(isize, isize, isize),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(a) => write!(f, "inp {}", a),
            Instruction::Add(a, b) => write!(f, "add {} {}", a, b),
            Instruction::Div(a, b) => write!(f, "div {} {}", a, b),
            Instruction::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Instruction::Mod(a, b) => write!(f, "mod {} {}", a, b),
            Instruction::Eql(a, b) => write!(f, "eql {} {}", a, b),
            Instruction::Setup(a, b) => write!(f, "setup {} {}", a, b),
            Instruction::Maama(r, t, a, b, c) => write!(
                f,
                "mul {} 0\nadd {} {}\nadd {} {}\nmul {} {}\nadd {} {}",
                r, r, a, r, b, r, c, t, r
            ),
            Instruction::Mamam(r, t, a, b, c) => write!(
                f,
                "mul {} 0\nadd {} {}\nmul {} {}\nadd {} {}\nmul {} {}",
                r, r, a, r, b, r, c, t, r
            ),
            Instruction::Oper(a, b, c) => write!(f, "oper z / {}, x + {}, y + {}", a, b, c),
        }
    }
}

fn parse_register(input: &str) -> IResult<&str, Operand> {
    let (input, reg) = one_of("wxyz")(input)?;
    let reg = reg.into();
    Ok((input, Operand::Register(reg)))
}

fn parse_value(input: &str) -> IResult<&str, Operand> {
    let (input, value) = parse_isize(input)?;
    Ok((input, Operand::Literal(value)))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((parse_value, parse_register))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, operator) = alt((
        tag("inp"),
        tag("add"),
        tag("div"),
        tag("mul"),
        tag("mod"),
        tag("eql"),
    ))(input)?;
    let (input, left) = ws(parse_register)(input)?;
    let left = match left {
        Operand::Register(r) => r,
        _ => unreachable!(),
    };

    if operator == "inp" {
        return Ok((input, Instruction::Inp(left)));
    }

    let (input, right) = ws(parse_operand)(input)?;
    match operator {
        "add" => Ok((input, Instruction::Add(left, right))),
        "div" => Ok((input, Instruction::Div(left, right))),
        "mul" => Ok((input, Instruction::Mul(left, right))),
        "mod" => Ok((input, Instruction::Mod(left, right))),
        "eql" => Ok((input, Instruction::Eql(left, right))),

        _ => unreachable!(),
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (_, inst) = parse_instruction(input).expect("known inputs");
        // dbg!(&inst);
        inst
    }
}

fn reduce_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut out = Vec::new();

    let mut instructions = instructions.iter();

    loop {
        let (_, _, _, _, a, b, _, _, _) = instructions.next_tuple().unwrap();
        let (_, _, _, _, _, _, c, _, _) = instructions.next_tuple().unwrap();

        let oper = match (a, b, c) {
            (Instruction::Div(_, a), Instruction::Add(_, b), Instruction::Add(_, c)) => {
                let (a, b, c) = match (a, b, c) {
                    (Operand::Literal(a), Operand::Literal(b), Operand::Literal(c)) => (a, b, c),
                    _ => unreachable!(),
                };

                Instruction::Oper(a.clone(), b.clone(), c.clone())
            }
            _ => unreachable!(),
        };
        out.push(oper);

        // let (_, _, _, _, a, b, _, _) = instructions.next_tuple().unwrap();

        // let setup = match (a, b) {
        //     (Instruction::Div(_, _a), Instruction::Add(_, _b)) => {
        //         Instruction::Setup(_a.clone(), _b.clone())
        //     }
        //     _ => unreachable!(),
        // };
        // out.push(setup);

        // let (_, a, b, c, d) = instructions.next_tuple().unwrap();
        // let oper = match (a, b, c, d) {
        //     (
        //         Instruction::Add(r, a),
        //         Instruction::Mul(_, b),
        //         Instruction::Add(_, c),
        //         Instruction::Mul(t, _),
        //     ) => Instruction::Mamam(r.clone(), t.clone(), a.clone(), b.clone(), c.clone()),
        //     _ => unreachable!(),
        // };
        // out.push(oper);

        // let (_, a, b, c, d) = instructions.next_tuple().unwrap();
        // let oper = match (a, b, c, d) {
        //     (
        //         Instruction::Add(r, a),
        //         Instruction::Add(_, b),
        //         Instruction::Mul(_, c),
        //         Instruction::Add(t, _),
        //     ) => Instruction::Maama(r.clone(), t.clone(), a.clone(), b.clone(), c.clone()),
        //     _ => unreachable!(),
        // };
        // out.push(oper);

        if instructions.len() == 0 {
            break;
        }
    }

    out
}

#[derive(Debug, Clone)]
struct State {
    memory: HashMap<Register, isize>,
    input: Vec<isize>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.memory.get(&Register::W).unwrap_or(&0);
        let x = self.memory.get(&Register::X).unwrap_or(&0);
        let y = self.memory.get(&Register::Y).unwrap_or(&0);
        let z = self.memory.get(&Register::Z).unwrap_or(&0);

        write!(
            f,
            "input: {:?}\nRegister::W  {}\nRegister::X  {}\nRegister::Y  {}\nRegister::Z  {}",
            self.input, w, x, y, z
        )
    }
}

impl State {
    fn new(input: Vec<isize>) -> Self {
        let mut memory = HashMap::new();
        memory.insert(Register::W, 0);
        memory.insert(Register::X, 0);
        memory.insert(Register::Y, 0);
        memory.insert(Register::Z, 0);

        Self { memory, input }
    }

    fn with_initial_state(self, memory: HashMap<Register, isize>) -> Self {
        let mut out = self;
        out.memory = memory;

        out
    }

    fn resolve(&self, operand: Operand) -> isize {
        let value = match operand {
            Operand::Literal(v) => v,
            Operand::Register(r) => self.memory.get(&r).unwrap().clone(),
        };

        value.clone()
    }

    fn apply(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Inp(reg) => {
                *self.memory.get_mut(reg).unwrap() = self.input.pop().unwrap();
            }
            Instruction::Add(reg, operand) => {
                let operand = self.resolve(operand.clone());
                let value = self.memory.get_mut(reg).unwrap();
                *value = *value + operand;
            }
            Instruction::Div(reg, operand) => {
                let operand = self.resolve(operand.clone());
                let value = self.memory.get_mut(reg).unwrap();
                *value = *value / operand;
            }
            Instruction::Mul(reg, operand) => {
                let operand = self.resolve(operand.clone());
                let value = self.memory.get_mut(reg).unwrap();
                *value = *value * operand;
            }
            Instruction::Mod(reg, operand) => {
                let operand = self.resolve(operand.clone());
                let value = self.memory.get_mut(reg).unwrap();
                *value = *value % operand;
            }
            Instruction::Eql(reg, operand) => {
                let operand = self.resolve(operand.clone());
                let value = self.memory.get_mut(reg).unwrap();
                *value = if *value == operand { 1 } else { 0 }
            }
            Instruction::Setup(a, b) => {
                let a = self.resolve(a.clone());
                let b = self.resolve(b.clone());
                let input = self.input.pop().unwrap();

                let z = self.memory.get_mut(&Register::Z).unwrap();
                let cmp = (*z % 26) + b;

                *z = *z / a;
                *self.memory.get_mut(&Register::W).unwrap() = input;
                *self.memory.get_mut(&Register::X).unwrap() = if cmp != input { 1 } else { 0 };
            }

            Instruction::Maama(reg, target, a, b, c) => {
                let a = self.resolve(a.clone());
                let b = self.resolve(b.clone());
                let c = self.resolve(c.clone());

                let v = (a + b) * c;
                let value = self.memory.get_mut(reg).unwrap();
                *value = v;

                let target = self.memory.get_mut(target).unwrap();
                *target = *target + v
            }
            Instruction::Mamam(reg, target, a, b, c) => {
                let a = self.resolve(a.clone());
                let b = self.resolve(b.clone());
                let c = self.resolve(c.clone());

                let v = (a * b) + c;
                let value = self.memory.get_mut(reg).unwrap();
                *value = v;

                let target = self.memory.get_mut(target).unwrap();
                *target = *target * v
            }
            Instruction::Oper(a, b, c) => {
                let w = self.input.pop().unwrap();
                let z = self.memory.get_mut(&Register::Z).unwrap();

                let x = ((*z % 26) + b) != w;
                *z = *z / a;
                *z = *z * if x { 26 } else { 1 };
                *z = *z + if x { w + c } else { 0 };
            }
        }
    }
}

struct Fourteener(usize);

impl Fourteener {
    fn has_zero(&self) -> bool {
        let mut x = self.0;
        while x > 0 {
            if x % 10 == 0 {
                return true;
            }
            x = x / 10;
        }
        false
    }

    fn new() -> Self {
        Self(99999999999999 + 1)
    }

    fn to_set(&self) -> Vec<isize> {
        let mut stack = Vec::new();
        let mut x = self.0 as isize;
        while x > 0 {
            stack.push(x % 10);
            x = x / 10;
        }
        stack.iter().rev().cloned().collect()
    }
}

impl IntoIterator for Fourteener {
    type Item = usize;
    type IntoIter = FourteenerIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        FourteenerIntoIter(self)
    }
}

// impl IntoParallelIterator for Fourteener {
//     type Iter = FourteenerIntoIter;

//     type Item = usize;

//     fn into_par_iter(self) -> Self::Iter {
//         todo!()
//     }
// }

struct FourteenerIntoIter(Fourteener);

impl Iterator for FourteenerIntoIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.0 .0 = self.0 .0 - 1;
            if !self.0.has_zero() {
                break;
            }
        }

        if self.0 .0 < 11111111111111 {
            None
        } else {
            Some(self.0 .0)
        }
    }
}

// impl ParallelIterator for FourteenerIntoIter {
//     type Item = usize;

//     fn drive_unindexed<C>(self, consumer: C) -> C::Result
//     where
//         C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
//     {
//         todo!()
//     }
// }

fn part_1(input: &str) {
    let instructions: Vec<Instruction> = input.trim().lines().map(|l| l.trim().into()).collect();
    let instructions = reduce_instructions(instructions);
    //dbg!(&instructions);

    // let handles = Arc::new(());
    // let counter = Arc::new(AtomicUsize::new(0));

    // let status_handle = handles.clone();
    // let status_counter = counter.clone();
    // thread::spawn(move || {
    //     let mut last_check = 0;
    //     loop {
    //         sleep(Duration::from_secs(2));
    //         let now = status_counter.load(Ordering::Relaxed);

    //         let delta = now - last_check;
    //         last_check = now;
    //         println!(
    //             "{} handles\n{:?} done\n{}/s\n",
    //             Arc::strong_count(&status_handle),
    //             status_counter,
    //             delta / 2
    //         );
    //     }
    // });

    // let result = Fourteener::new()
    //     .into_iter()
    //     .par_bridge()
    //     .map(|candidate| {
    //         let handle = handles.clone();
    //         let set = Fourteener(candidate).to_set();
    //         let mut state = State::new(set);
    //         for instruction in &instructions {
    //             //println!("{}\n{}\n\n", &state, instruction);
    //             state.apply(instruction);
    //         }
    //         counter.fetch_add(1, Ordering::Relaxed);
    //         (candidate, state)
    //     })
    //     .find_first(|(candidate, state)| {
    //         state.memory.get(&Register::Z).unwrap() == &0
    //         //false
    //     });

    // dbg!(result);

    for candidate in Fourteener::new().into_iter().take(2) {
        let set = Fourteener(candidate).to_set();

        println!("\n\n\nchecking {:?} -> {}", &set, candidate);

        let mut state = State::new(set);
        println!("{}\n", state);
        for instruction in &instructions {
            println!("{}", instruction);
            state.apply(instruction);
            println!("{}\n", state);
        }

        let z = state.memory.get(&Register::Z).unwrap();

        // println!("checking {}, z = {}", candidate, z);

        if z == &0 {
            println!("{}\n{}\n", state, candidate);
            break;
        }
    }
}

fn part_1_take_2(input: &str) {
    let instructions: Vec<Instruction> = input.trim().lines().map(|l| l.trim().into()).collect();
    let instructions = reduce_instructions(instructions);

    let instruction = instructions.first().unwrap();
    for i in 1..=9 {
        let mut state = State::new(vec![i]);
        state.apply(instruction);
        println!("{} => \n{}\n", i, state);
    }
}

fn binary_example() {
    let input = r#"inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2"#;

    let instructions: Vec<Instruction> = input.trim().lines().map(|l| l.trim().into()).collect();

    for i in 0..16 {
        let mut state = State::new(vec![i]);
        for instruction in &instructions {
            state.apply(instruction);
        }
        println!("\n{}\n{}", i, state);
    }
}

fn main() {
    let input = input_store::get_input(2021, 24);

    // let input = r#"inp z
    // inp x
    // mul z 3
    // eql z x"#;

    part_1_take_2(&input);

    println!("part_1 => {}", "not done");
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
