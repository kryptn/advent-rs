use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 07;

enum Operator {
    Add,
    Multiply,
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
    fn find(&self) -> usize {
        let two: usize = 2;
        let t = two.pow(self.given.len() as u32 - 1);

        for mut oper in 1..=t {
            let mut value = self.given[0];

            // for each remaining given number
            for i in 1..self.given.len() {

                // get the right-most bit in the operator and shift
                let operator = oper & 1;
                oper >>= 1;

                match operator {
                    0 => value += self.given[i],
                    1 => value *= self.given[i],
                    _ => panic!("Invalid operator"),
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

    let part_1 = problems.iter().map(|p| p.find()).sum::<usize>();

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
