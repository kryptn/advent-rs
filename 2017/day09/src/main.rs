use std::collections::{HashMap, HashSet};

use advent::input_store;
use itertools::Itertools;
use regex::Regex;

fn clean_garbage(input: &str) -> String {
    let ignore = Regex::new("!.").unwrap();
    let input = ignore.replace_all(input, "");

    let garbage = Regex::new("<.*?>").unwrap();
    let input = garbage.replace_all(&input, "");

    input.to_string()
}

fn collect_garbage(input: &str) -> String {
    let ignore = Regex::new("!.").unwrap();
    let input = ignore.replace_all(input, "");

    let garbage = Regex::new("<(.*?)>").unwrap();

    let captures: Vec<String> = garbage
        .captures_iter(&input)
        .map(|m| m.get(1).unwrap().as_str().to_string())
        .collect();

    captures.join("")
}

fn score(input: &str) -> usize {
    let mut level = 0;
    let mut score = 0;
    for c in input.chars() {
        match c {
            '{' => {
                level += 1;
                score += level;
            }
            '}' => level -= 1,
            _ => {}
        }
    }

    score
}

fn main() {
    let input = input_store::get_input(2017, 09);

    println!("part_1 => {}", score(&clean_garbage(input.trim())));
    println!("part_2 => {}", collect_garbage(input.trim()).len());
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
    #[case("<>", "")]
    #[case("<random characters>", "")]
    #[case("<<<<>", "")]
    #[case("<{!>}>", "")]
    #[case("<!!>", "")]
    #[case("<!!!>>", "")]
    #[case(r#"<{o"i!a,<{i<a>"#, "")]

    fn clean_tests(#[case] given: &str, #[case] expected: String) {
        assert_eq!(clean_garbage(given), expected);
    }

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 6)]
    #[case("{{},{}}", 5)]
    #[case("{{{},{},{{}}}}", 16)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)]
    #[case("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9)]
    #[case("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)]
    fn score_tests(#[case] given: &str, #[case] expected: usize) {
        let cleaned = clean_garbage(given);
        assert_eq!(score(&cleaned), expected);
    }

    #[rstest]
    #[case("<>", 0)]
    #[case("<random characters>", 17)]
    #[case("<<<<>", 3)]
    #[case("<{!>}>", 2)]
    #[case("<!!>", 0)]
    #[case("<!!!>>", 0)]
    #[case(r#"<{o"i!a,<{i<a>"#, 10)]
    fn p2_tests(#[case] given: &str, #[case] expected: usize) {
        let collected = collect_garbage(given);

        println!("{}, {}", given, collected);
        assert_eq!(collected.len(), expected);
    }
}
