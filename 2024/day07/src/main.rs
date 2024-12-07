use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 07;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Problem {
    goal: usize,
    given: Vec<usize>,
}

impl From<&str> for Problem {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(":").collect();
        let goal = parts[0].parse().unwrap();
        let given = parts[1]
            .trim()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();

        Self { goal, given }
    }
}

impl Problem {
    fn find(&self, operators: Vec<Operator>) -> usize {
        for oper in std::iter::repeat(operators)
            .take(self.given.len() - 1)
            .multi_cartesian_product()
        {
            let mut value = self.given[0];

            for (g, operator) in self.given.iter().skip(1).zip(oper.iter()) {
                match operator {
                    Operator::Add => value += g,
                    Operator::Multiply => value *= g,
                    Operator::Concatenate => value = format!("{}{}", value, g).parse().unwrap(),
                }
            }

            if value == self.goal {
                return self.goal;
            }
        }

        0
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // "#;

    let problems = input
        .lines()
        .map(|p| Problem::from(p))
        .collect::<Vec<Problem>>();

    let part_1 = problems
        .iter()
        .map(|p| p.find(vec![Operator::Add, Operator::Multiply]))
        .sum::<usize>();

    println!("part_1 => {}", part_1);

    let part_2 = problems
        .iter()
        .map(|p| {
            p.find(vec![
                Operator::Add,
                Operator::Multiply,
                Operator::Concatenate,
            ])
        })
        .sum::<usize>();
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
