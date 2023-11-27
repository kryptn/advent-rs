use std::{collections::HashMap, fmt::Debug};

use advent::input_store;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    IResult,
};

#[derive(Debug)]
struct Rule {
    catalyst: String,
    pairs: (String, String),
    element: String,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, before) = take(1usize)(input)?;
    let (input, after) = take(1usize)(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, element) = take(1usize)(input)?;

    let catalyst = format!("{}{}", before, after);
    let left = format!("{}{}", before, element);
    let right = format!("{}{}", element, after);
    let element = element.to_string();

    Ok((
        input,
        Rule {
            catalyst,
            pairs: (left, right),
            element,
        },
    ))
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        let (_, rule) = parse_rule(input).unwrap();
        rule
    }
}

fn insert(polymer: &mut HashMap<String, isize>, rules: &Vec<Rule>) {
    let mut changes: HashMap<&String, isize> = HashMap::new();

    for rule in rules {
        let catalyst = polymer.get(&rule.catalyst).unwrap().clone();

        *changes.entry(&rule.catalyst).or_default() -= catalyst;
        *changes.entry(&rule.pairs.0).or_default() += catalyst;
        *changes.entry(&rule.pairs.1).or_default() += catalyst;
        *changes.entry(&rule.element).or_default() += catalyst;
    }

    for (key, delta) in changes {
        *polymer.get_mut(key).unwrap() += delta;
    }
}

fn get_score(polymer: &HashMap<String, isize>) -> isize {
    let elements: HashMap<String, isize> = polymer
        .iter()
        .filter(|(k, v)| k.len() == 1 && *v > &0)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    println!("{}", elements.values().sum::<isize>());

    elements.values().max().unwrap() - elements.values().min().unwrap()
}

fn main() {
    let input = input_store::get_input(2021, 14);

    let mut input_split = input.trim().split("\n\n");

    let given = input_split.next().unwrap();

    let rules: Vec<Rule> = {
        let r = input_split.next().unwrap();
        r.lines().map(|l| l.into()).collect()
    };

    let mut polymer: HashMap<String, isize> =
        rules.iter().map(|r| (r.catalyst.clone(), 0)).collect();

    for (l, r) in given.chars().tuple_windows() {
        let pair: String = [l, r].iter().collect();
        *polymer.get_mut(&pair).unwrap() += 1;
    }

    for l in 'A'..='Z' {
        polymer.insert(l.to_string(), 0);
    }

    for l in given.chars() {
        let element = l.to_string();
        *polymer.get_mut(&element).unwrap() += 1;
    }

    for _ in 0..10 {
        insert(&mut polymer, &rules);
    }

    let part_1 = get_score(&polymer);

    println!("part_1 => {}", part_1);

    for _ in 0..30 {
        insert(&mut polymer, &rules);
    }

    let part_2 = get_score(&polymer);

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
