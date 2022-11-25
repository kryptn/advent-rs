use std::collections::{HashMap, HashSet};

use advent::input_store;

enum Conditional {
    Lt(String, isize),
    Lte(String, isize),
    Gt(String, isize),
    Gte(String, isize),
    Eq(String, isize),
    Ne(String, isize),
}

impl From<(&str, &str, &str)> for Conditional {
    fn from((sub, cond, by): (&str, &str, &str)) -> Self {
        // dbg!(sub, cond, by);
        let sub = sub.trim().to_string();
        let by = by.trim().parse().unwrap();
        match cond.trim() {
            "<" => Self::Lt(sub, by),
            "<=" => Self::Lte(sub, by),
            ">" => Self::Gt(sub, by),
            ">=" => Self::Gte(sub, by),
            "==" => Self::Eq(sub, by),
            "!=" => Self::Ne(sub, by),
            _ => panic!("unexpected on From::Conditional"),
        }
    }
}

enum Oper {
    Inc(String, isize),
    Dec(String, isize),
}

impl From<(&str, &str, &str)> for Oper {
    fn from((target, oper, by): (&str, &str, &str)) -> Self {
        // dbg!(target, oper, by);
        let target = target.to_string();
        let by = by.trim().parse().unwrap();
        match oper.trim() {
            "inc" => Self::Inc(target, by),
            "dec" => Self::Dec(target, by),
            _ => panic!("unexpected on From::Oper"),
        }
    }
}

struct Instruction {
    oper: Oper,
    cond: Conditional,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        //let (target, oper, oper_by, _, cond_target, cond_oper, cond_by) =
        let it: Vec<&str> = input.trim().split_whitespace().collect();
        let oper = (it[0], it[1], it[2]).into();
        let cond = (it[4], it[5], it[6]).into();
        Self { oper, cond }
    }
}

struct Memory(HashMap<String, isize>);

impl Memory {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn test(&self, cond: Conditional) -> bool {
        match cond {
            Conditional::Lt(target, check) => self.0.get(&target).unwrap_or(&0) < &check,
            Conditional::Lte(target, check) => self.0.get(&target).unwrap_or(&0) <= &check,
            Conditional::Gt(target, check) => self.0.get(&target).unwrap_or(&0) > &check,
            Conditional::Gte(target, check) => self.0.get(&target).unwrap_or(&0) >= &check,
            Conditional::Eq(target, check) => self.0.get(&target).unwrap_or(&0) == &check,
            Conditional::Ne(target, check) => self.0.get(&target).unwrap_or(&0) != &check,
        }
    }

    fn apply(&mut self, oper: Oper) {
        let (target, delta) = match oper {
            Oper::Inc(target, by) => (target, by),
            Oper::Dec(target, by) => (target, -by),
        };

        *self.0.entry(target).or_insert(0) += delta;
    }

    fn run(&mut self, inst: Instruction) {
        if self.test(inst.cond) {
            self.apply(inst.oper);
        }
    }

    fn highest(&self) -> isize {
        match self.0.values().max() {
            Some(v) => v.clone(),
            None => 0,
        }
    }
}

fn main() {
    let input = input_store::get_input(2017, 08);
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut memory = Memory::new();

    let mut highs = Vec::new();
    for instruction in instructions {
        memory.run(instruction);
        highs.push(memory.highest());
    }

    let part_1 = memory.highest();
    println!("part 1: {}", part_1);

    let part_2 = highs.iter().max().unwrap();
    println!("part 2: {}", part_2);
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
