use std::collections::HashSet;

use advent::input_store;
use itertools::Itertools;

fn main() {
    let input = input_store::get_input(2017, 04);

    let pw_list: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.trim().split_ascii_whitespace().collect())
        .collect();

    let valid = pw_list
        .iter()
        .filter(|phrase| {
            let set: HashSet<&str> = phrase.iter().cloned().collect();
            set.len() == phrase.len()
        })
        .count();

    println!("part_1 => {}", valid);

    let valid = pw_list
        .iter()
        .map(|phrase| {
            let sorted_words: Vec<String> = phrase
                .iter()
                .map(|word| word.chars().sorted().collect())
                .collect();
            sorted_words
        })
        .filter(|phrase| {
            let set: HashSet<String> = phrase.iter().cloned().collect();
            set.len() == phrase.len()
        })
        .count();

    println!("part_2 => {}", valid);
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
