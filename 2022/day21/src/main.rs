use std::{cell::RefCell, collections::HashMap};

use advent::input_store;

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<&str> for Operator {
    fn from(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!("nothing else expected"),
        }
    }
}

impl Operator {
    fn apply(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    left: String,
    right: String,
    operator: Operator,
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: RefCell<Option<isize>>,
    operation: Option<Operation>,
}

impl Monkey {
    fn figure_value(&self, monkeys: &Monkeys) {
        if let Some(operation) = &self.operation {
            let left = monkeys.figure(&operation.left);
            let right = monkeys.figure(&operation.right);
            let value = operation.operator.apply(left, right);
            self.value.replace(Some(value));
        }
    }
}

impl<'a> From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let cleaned = input.replace(":", "");
        let split: Vec<_> = cleaned.trim().split_whitespace().collect();
        let name = split[0].to_string();
        if split.len() == 2 {
            let value = Some(split[1].parse().unwrap()).into();
            let operation = None;
            Self {
                name,
                value,
                operation,
            }
        } else {
            let left = split[1].to_string();
            let right = split[3].to_string();
            let operator = split[2].into();
            let operation = Some(Operation {
                left,
                right,
                operator,
            });
            Self {
                name,
                value: None.into(),
                operation,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkeys(HashMap<String, Monkey>);

impl Monkeys {
    fn figure(&self, name: &String) -> isize {
        let monkey = self.0.get(name).expect("");
        monkey.figure_value(&self);
        monkey.value.borrow().unwrap().clone()
    }

    fn root_eq(&self, value: isize) -> (std::cmp::Ordering, isize) {
        let humn = "humn".to_string();
        let root = "root".to_string();

        let root = self.0.get(&root).expect("");
        let humn = self.0.get(&humn).expect("");

        humn.value.replace(Some(value));

        let operation = root.operation.clone().expect("");
        let left = self.0.get(&operation.left).unwrap();
        let right = self.0.get(&operation.right).unwrap();

        left.figure_value(&self);
        right.figure_value(&self);

        let left_value = left.value.borrow().unwrap();
        let right_value = right.value.borrow().unwrap();

        let result = left_value.cmp(&right_value);
        let delta = left_value - right_value;

        (result, delta)
    }
}

fn main() {
    let input = input_store::get_input(2022, 21);
    // let input = r#"root: pppw + sjmn
    // dbpl: 5
    // cczh: sllz + lgvd
    // zczc: 2
    // ptdq: humn - dvpt
    // dvpt: 3
    // lfqf: 4
    // humn: 5
    // ljgn: 2
    // sjmn: drzm * dbpl
    // sllz: 4
    // pppw: cczh / lfqf
    // lgvd: ljgn * ptdq
    // drzm: hmdt - zczc
    // hmdt: 32"#;

    let monkeys: HashMap<String, Monkey> = input
        .trim()
        .lines()
        .map(|line| {
            let monkey = Monkey::from(line);
            (monkey.name.clone(), monkey)
        })
        .collect();

    let monkeys_p1 = Monkeys(monkeys.clone());

    let root = "root".to_string();
    let root = monkeys_p1.figure(&root);

    println!("part_1 => {}", root);

    let monkeys_p2 = Monkeys(monkeys.clone());

    let mut x = 0;
    let mut delta = 1;

    loop {
        let (_, d) = monkeys_p2.root_eq(x);
        // println!("x: {x}, delta: {delta}, d: {d}");

        match 0.cmp(&d) {
            std::cmp::Ordering::Less => {
                if delta <= 0 {
                    delta = 1
                }
                delta += delta
            }
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => {
                if delta >= 0 {
                    delta = -1
                }
                delta += delta
            }
        }
        x += delta;
    }

    loop {
        let (cmp, _d) = monkeys_p2.root_eq(x - 1);
        // println!("x: {x}, delta: -1, d: {d}");

        match cmp {
            std::cmp::Ordering::Equal => x -= 1,
            _ => {
                break;
            }
        }
    }

    println!("part_2 => {}", x);
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
