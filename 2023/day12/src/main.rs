use std::{collections::HashMap, default};

use advent::input_store;
use itertools::Itertools;
use regex::Regex;

const YEAR: usize = 2023;
const DAY: usize = 12;

fn parse_input(input: &str) -> (String, Vec<usize>) {
    let sections: Vec<_> = input.split_ascii_whitespace().collect();

    let actual_input = sections.first().unwrap();
    let groups = sections
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    (actual_input.to_string(), groups)
}

fn indexed_replacements<T>(indexes: &Vec<usize>, replacements: Vec<T>) -> Vec<HashMap<usize, T>>
where
    T: Clone,
{
    let replacements = vec![replacements; indexes.len()];
    let mut out = vec![];
    for repl in replacements.iter().multi_cartesian_product() {
        out.push(
            indexes
                .iter()
                .zip(repl.into_iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect(),
        );
    }
    out
}

fn replace_with(s: String, replacements: HashMap<usize, char>) -> String {
    let mut out = String::new();
    for (idx, c) in s.chars().enumerate() {
        if let Some(replacement) = replacements.get(&idx) {
            out.push(*replacement);
        } else {
            out.push(c);
        }
    }
    out
}

fn part_1(input: &str, expected: Vec<usize>) -> usize {
    let pattern = {
        let mut sections = expected.iter().map(|v| format!("#{{{v}}}"));
        let joined = sections.join(r"\.+");
        let pattern = format!(r"^\.*{}\.*$", joined);
        // println!("pattern: {}", pattern);
        Regex::new(&pattern).unwrap()
    };

    // println!("pattern: {:?}", pattern);
    // println!("{input}");
    let indexes = input
        .chars()
        .enumerate()
        .filter(|(_, c)| c == &'?')
        .map(|(idx, _)| idx)
        .collect();

    let mut out = 0;
    for repl in indexed_replacements(&indexes, vec!['#', '.']) {
        let replaced = replace_with(input.to_string(), repl);
        // print!("{replaced} ");
        if pattern.is_match(&replaced) {
            // print!("match");
            out += 1;
        }
        // println!();
    }

    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let inputs: Vec<_> = input
        .trim()
        .lines()
        .map(|l| parse_input(l.trim()))
        .collect();

    let p1 = inputs
        .iter()
        .map(|(input, expected)| part_1(input, expected.clone()))
        .sum::<usize>();

    println!("part_1 => {}", p1);
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
    #[case("012345", vec![(0, '.')], ".12345")]
    #[case("012345", vec![(1, '.')], "0.2345")]
    #[case("012345", vec![(0, '.'), (1, '.')], "..2345")]
    fn test_indexed_replacements(
        #[case] input: &str,
        #[case] replacements: Vec<(usize, char)>,
        #[case] expected: &str,
    ) {
        let replacements = replacements.iter().map(|(idx, c)| (*idx, *c)).collect();
        let replaced = replace_with(input.to_string(), replacements);
        assert_eq!(replaced, expected);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn p1_tests(#[case] given: &str, #[case] expected: usize) {
        let (input, sections) = parse_input(given);

        assert_eq!(part_1(&input, sections), expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
