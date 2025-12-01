use std::collections::{HashMap, HashSet};

use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 24;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Oper {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Oper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Oper::And => write!(f, "AND"),
            Oper::Or => write!(f, "OR"),
            Oper::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

struct Gate {
    operation: Oper,
    left: String,
    right: String,
    output: String,
}

impl Gate {
    fn eval(&self, wires: &HashMap<String, bool>) -> Option<bool> {
        let left = wires.get(&self.left)?;
        let right = wires.get(&self.right)?;

        match self.operation {
            Oper::And => Some(left & right),
            Oper::Or => Some(left | right),
            Oper::Xor => Some(left ^ right),
        }
    }
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.left, self.operation, self.right, self.output
        )
    }
}

impl From<&str> for Gate {
    fn from(s: &str) -> Self {
        let (left, operator, right, _, output) = s.trim().split(" ").collect_tuple().unwrap();

        let operator = match operator {
            "AND" => Oper::And,
            "OR" => Oper::Or,
            "XOR" => Oper::Xor,
            _ => panic!("Invalid operator"),
        };

        Self {
            operation: operator,
            left: left.to_string(),
            right: right.to_string(),
            output: output.to_string(),
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
//     let input = r#"x00: 1
// x01: 0
// x02: 1
// x03: 1
// x04: 0
// y00: 1
// y01: 1
// y02: 1
// y03: 1
// y04: 1

// ntg XOR fgs -> mjb
// y02 OR x01 -> tnw
// kwq OR kpj -> z05
// x00 OR x03 -> fst
// tgd XOR rvg -> z01
// vdt OR tnw -> bfw
// bfw AND frj -> z10
// ffh OR nrd -> bqk
// y00 AND y03 -> djm
// y03 OR y00 -> psh
// bqk OR frj -> z08
// tnw OR fst -> frj
// gnj AND tgd -> z11
// bfw XOR mjb -> z00
// x03 OR x00 -> vdt
// gnj AND wpb -> z02
// x04 AND y00 -> kjc
// djm OR pbm -> qhw
// nrd AND vdt -> hwm
// kjc AND fst -> rvg
// y04 OR y02 -> fgs
// y01 AND x02 -> pbm
// ntg OR kjc -> kwq
// psh XOR fgs -> tgd
// qhw XOR tgd -> z09
// pbm OR djm -> kpj
// x03 XOR y03 -> ffh
// x00 XOR y04 -> ntg
// bfw OR bqk -> z06
// nrd XOR fgs -> wpb
// frj XOR qhw -> z04
// bqk OR frj -> z07
// y03 OR x01 -> nrd
// hwm AND bqk -> z03
// tgd XOR rvg -> z12
// tnw OR pbm -> gnj"#;

    let (initial_state, input) = input.split("\n\n").collect_tuple().unwrap();

    let mut wire_states: HashMap<String, bool> = initial_state
        .trim()
        .lines()
        .map(|line| {
            let (wire, value) = line.trim().split(": ").collect_tuple().unwrap();
            (wire.to_string(), value == "1")
        })
        .collect();

    let gates = input
        .trim()
        .lines()
        .map(|line| Gate::from(line))
        .collect::<Vec<_>>();

    let mut known_wires: HashSet<String> = wire_states.keys().map(|k| k.clone()).collect();

    let mut dependency_ordered_gates = vec![];

    while dependency_ordered_gates.len() < gates.len() {
        // println!("{:?} < {:?}", dependency_ordered_gates.len(), gates.len());
        // println!("{:?}", known_wires);
        for gate in gates.iter() {
            // let gate_within = dependency_ordered_gates.contains(gate);
            // let left_known = known_wires.contains(&gate.left);
            // let right_known = known_wires.contains(&gate.right);

            // println!("{:?} {:?} {:?}", gate_within, left_known, right_known);

            if !dependency_ordered_gates.contains(gate)
                && known_wires.contains(&gate.left)
                && known_wires.contains(&gate.right)
            {
                known_wires.insert(gate.output.clone());
                dependency_ordered_gates.push(gate.clone());
            }
        }
    }
    // println!("{:?} < {:?}", dependency_ordered_gates.len(), gates.len());
    // println!("{:?}", known_wires);

    for gate in dependency_ordered_gates {
        let value = gate.eval(&wire_states).unwrap();
        wire_states.insert(gate.output, value);
    }

    let part_1 = wire_states
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted()
        .rev()
        .fold(0, |acc, (k, v)| -> i64 {
            println!("{:?} {:?}, acc: {:#b}", k, v, acc);
            acc << 1 | if *v { 1 } else { 0 }
        });

    println!("part_1 => {}", part_1);
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
