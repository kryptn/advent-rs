use advent::input_store;
use advent_toolbox::{parser_helpers::just_numbers, spatial::Coordinate};

const YEAR: usize = 2024;
const DAY: usize = 13;

#[derive(Debug)]
struct Machine {
    a: Coordinate,
    b: Coordinate,
    goal: Coordinate,
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let numbers: Vec<usize> = just_numbers(input);
        let mut numbers = numbers.into_iter();
        let ax = numbers.next().unwrap();
        let ay = numbers.next().unwrap();
        let bx = numbers.next().unwrap();
        let by = numbers.next().unwrap();
        let cx = numbers.next().unwrap();
        let cy = numbers.next().unwrap();

        Machine {
            a: (ax, ay).into(),
            b: (bx, by).into(),
            goal: (cx, cy).into(),
        }
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Button A: X+{}, Y+{}\n", self.a.x, self.a.y)?;
        write!(f, "Button B: X+{}, Y+{}\n", self.b.x, self.b.y)?;
        write!(f, "Prize: X={}, Y={}\n", self.goal.x, self.goal.y)
    }
}

impl Machine {
    fn solve_better(&self) -> Option<Coordinate> {
        let det = self.a.x * self.b.y - self.a.y * self.b.x;

        let a_det = self.goal.x * self.b.y - self.goal.y * self.b.x;
        let a = a_det / det;
        let b_det = self.a.x * self.goal.y - self.a.y * self.goal.x;
        let b = b_det / det;

        let a_verify = self.a.x * a + self.b.x * b;
        let b_verify = self.a.y * a + self.b.y * b;

        if a_verify != self.goal.x || b_verify != self.goal.y {
            return None;
        }

        Some(Coordinate { x: a, y: b })
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279"#;

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|machine| Machine::from(machine))
        .collect();

    let part_1: isize = machines
        .iter()
        .filter_map(|machine| machine.solve_better())
        .map(|coord| coord.x * 3 + coord.y)
        .sum();

    const P2: Coordinate = Coordinate {
        x: 10000000000000,
        y: 10000000000000,
    };
    println!("part_1 => {}", part_1);

    let machines: Vec<Machine> = machines
        .into_iter()
        .map(|m| Machine {
            goal: m.goal + P2,
            ..m
        })
        .collect();

    let part_2: isize = machines
        .iter()
        .filter_map(|machine| machine.solve_better())
        .map(|coord: Coordinate| coord.x * 3 + coord.y)
        .sum();

    println!("part_2 => {}", part_2);
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
