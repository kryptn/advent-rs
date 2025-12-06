use advent::input_store;
use advent_toolbox::parser_helpers::just_numbers;
use itertools::Itertools;

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

fn part_1(input: &str) -> isize {
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

    part_1
}

fn part_2(input: &str) -> isize {
    let mut lines = input.lines().rev();
    let operations = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Operation::from(c))
        .collect::<Vec<Operation>>();

    let rest = lines
        .rev()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = rest.iter().map(|row| row.len()).max().unwrap();

    let mut rewritten = vec![];
    for column in 0..width {
        let mut chars = String::new();
        for row in 0..rest.len() {
            if column >= rest[row].len() {
                chars.push(' ');
                continue;
            }

            chars.push(rest[row][column]);
        }
        if chars.trim().is_empty() {
            rewritten.push(String::new());
        } else {
            rewritten.push(chars);
        }
    }

    let rebuilt = rewritten.iter().join("\n");
    let sets = rebuilt
        .split("\n\n")
        .map(|block| {
            // println!("block:\n{}\n---", block);
            let numbers = just_numbers::<isize>(block);
            // println!("numbers: {:?}", numbers);
            numbers
        })
        .collect::<Vec<_>>();

    // println!("rebuilt:\n\n---\n{}\n---", rebuilt);

    // println!(
    //     "sets.len(): {}, operations.len(): {}",
    //     sets.len(),
    //     operations.len()
    // );
    assert!(sets.len() == operations.len());

    let part_2 = operations
        .iter()
        .enumerate()
        .map(|(idx, op)| {
            let n = sets[idx].clone();

            match op {
                Operation::Add => n.iter().sum::<isize>(),
                Operation::Multiply => n.iter().product::<isize>(),
            }
        })
        .sum::<isize>();

    part_2
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

//     let input = r#"123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +"#;

    let part_1 = part_1(&input);
    println!("part_1 => {}", part_1);

    let part_2 = part_2(&input);
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
