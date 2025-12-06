use advent::input_store;
use advent_toolbox::parser_helpers::just_numbers;

const YEAR: usize = 2025;
const DAY: usize = 06;

enum Operation {
    Add,
    Multiply,
}

impl From<char> for Operation {
    fn from(c: char) -> Self {
        match c {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("unknown operation char {}", c),
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

//     let input = r#"123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +"#;

    let mut lines = input.trim().lines().rev();
    let operations = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Operation::from(c))
        .collect::<Vec<Operation>>();
    let numbers = lines
        .rev()
        .map(|line| just_numbers::<isize>(line))
        .collect::<Vec<_>>();

    let part_1 = operations
        .iter()
        .enumerate()
        .map(|(idx, op)| {
            let n = numbers.iter().map(|ns| ns[idx]).collect::<Vec<isize>>();

            match op {
                Operation::Add => n.iter().sum::<isize>(),
                Operation::Multiply => n.iter().product::<isize>(),
            }
        })
        .sum::<isize>();

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
