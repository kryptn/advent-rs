use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2025;
const DAY: usize = 03;

fn left_most_max(bank: &str, k: usize) -> u64 {
    let indexed_chars = bank[0..=bank.len() - k]
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as u64))
        .sorted_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        })
        .collect::<Vec<_>>();
    let max_char = indexed_chars.iter().max_by_key(|(_, v)| *v).unwrap();

    if k == 1 {
        return max_char.1;
    } else {
        let remaining = &bank[max_char.0 + 1..];
        return max_char.1 * 10u64.pow((k - 1) as u32) + left_most_max(remaining, k - 1);
    }
}

const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = TEST_INPUT.to_string();

    let part_1 = input
        .trim()
        .lines()
        .map(|line| left_most_max(line, 2))
        .sum::<u64>();

    let part_2 = input
        .trim()
        .lines()
        .map(|line| left_most_max(line, 12))
        .sum::<u64>();

    println!("part_1 => {}", part_1);
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
