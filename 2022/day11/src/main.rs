use std::collections::VecDeque;

use advent::input_store;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Operation {
    Add(i64),
    Subtract(i64),
    Multiply(i64),
    Divide(i64),
    Modulus(i64),
    Exponent(i64),
}

impl Operation {
    fn apply(&self, other: i64) -> i64 {
        match self {
            Operation::Add(v) => other + v,
            Operation::Subtract(v) => other - v,
            Operation::Multiply(v) => other * v,
            Operation::Divide(v) => other / v,
            Operation::Modulus(v) => other % v,
            Operation::Exponent(v) => other.pow((*v) as u32),
        }
    }
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let split: Vec<_> = line.trim().split_whitespace().collect();
        let operand = match split[5].parse() {
            Ok(v) => v,
            Err(_) => return Self::Exponent(2),
        };
        match split[4] {
            "+" => Self::Add(operand),
            "-" => Self::Subtract(operand),
            "*" => Self::Multiply(operand),
            "/" => Self::Divide(operand),
            "%" => Self::Modulus(operand),

            _ => panic!("nothing else expected"),
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    oper: Operation,
    target_if_true: usize,
    target_if_false: usize,
}

impl Test {
    fn check(&self, worry: i64) -> usize {
        if self.oper.apply(worry) == 0 {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
}

impl From<&str> for Test {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let operand = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let oper = Operation::Modulus(operand);
        let target_if_true = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let target_if_false = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            oper,
            target_if_true,
            target_if_false,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    number: usize,
    items: VecDeque<i64>,
    operation: Operation,
    post_inspection: Operation,
    test: Test,

    inspections: usize,
}

impl Monkey {
    fn inspect(&mut self) -> (usize, i64) {
        let worry = self.items.pop_front().unwrap();
        let worry = self.operation.apply(worry);

        let worry = self.post_inspection.apply(worry);

        let target = self.test.check(worry);

        self.inspections += 1;

        (target, worry)
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.trim().lines();

        let number = lines
            .next()
            .unwrap()
            .trim()
            .trim_end_matches(":")
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let items = {
            let split: Vec<&str> = lines.next().unwrap().trim().split_whitespace().collect();
            split[2..]
                .iter()
                .map(|a| a.trim_end_matches(",").parse().unwrap())
                .collect()
        };

        let operation = lines.next().unwrap().into();

        let post_inspection = Operation::Divide(3);

        let test = {
            let remaining: Vec<&str> = lines.collect();
            let t = remaining.join("\n");
            t.as_str().into()
        };

        let inspections = 0;

        Self {
            number,
            items,
            operation,
            post_inspection,
            test,
            inspections,
        }
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.number,
            self.items.iter().join(", ")
        )
    }
}

#[derive(Clone, Debug)]
struct Monkeys(Vec<Monkey>);

impl From<&str> for Monkeys {
    fn from(input: &str) -> Self {
        let monkeys: Vec<Monkey> = input
            .trim()
            .split("\n\n")
            .map(|chunk| chunk.into())
            .collect();
        Self(monkeys)
    }
}

impl Monkeys {
    fn round(&mut self) {
        for idx in 0..self.0.len() {
            let targets = {
                let monkey = self.0.get_mut(idx).unwrap();
                let mut targets = Vec::new();
                while !monkey.items.is_empty() {
                    let result = monkey.inspect();
                    targets.push(result);
                }
                targets
            };

            for (target, worry) in targets {
                self.0[target].items.push_back(worry);
            }
        }
    }

    fn rounds(&mut self, k: usize) {
        for _ in 0..k {
            self.round();
        }
    }

    fn business(&self) -> usize {
        let inspections: Vec<usize> = self
            .0
            .iter()
            .map(|m| m.inspections)
            .sorted()
            .rev()
            .collect();

        inspections[0] * inspections[1]
    }

    fn get_test_values(&self) -> Vec<i64> {
        self.0
            .iter()
            .map(|m| match m.test.oper {
                Operation::Modulus(operand) => operand,
                _ => panic!(),
            })
            .collect()
    }

    fn set_post_inspection_operation(&mut self, oper: Operation) {
        for monkey in self.0.iter_mut() {
            monkey.post_inspection = oper.clone()
        }
    }
}

impl std::fmt::Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let each = self.0.iter().join("\n");
        write!(f, "{}", each)
    }
}

fn main() {
    let input = input_store::get_input(2022, 11);

    let mut monkeys: Monkeys = input.as_str().into();
    monkeys.rounds(20);
    println!("part_1 => {}", monkeys.business());

    let mut monkeys: Monkeys = input.as_str().into();

    let increased_worry = Operation::Modulus(
        monkeys
            .get_test_values()
            .into_iter()
            .reduce(|a, b| a * b)
            .unwrap(),
    );

    monkeys.set_post_inspection_operation(increased_worry);

    monkeys.rounds(10000);

    println!("part_2 => {}", monkeys.business());
}

#[cfg(test)]
mod test {

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
