use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 02;

fn is_gradual(numbers: &Vec<isize>) -> bool {
    let deltas: Vec<isize> = numbers.windows(2).map(|w| w[1] - w[0]).collect();

    if deltas.iter().any(|d| d.abs() < 1 || d.abs() > 3) {
        return false;
    }

    let directions = deltas.iter().map(|d| d / d.abs()).unique().count();
    if directions > 1 {
        return false;
    }

    true
}

fn is_gradual_enough(numbers: &Vec<isize>) -> bool {
    if is_gradual(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(i);
        if is_gradual(&new_numbers) {
            return true;
        }
    }

    false
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9"#;

    let reports: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let part_1 = reports.iter().filter(|r| is_gradual(r)).count();
    println!("part_1 => {}", part_1);

    let part_2 = reports.iter().filter(|r| is_gradual_enough(r)).count();
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
