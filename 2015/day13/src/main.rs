use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    pin::Pin,
    str::FromStr,
};

use advent::fetch;
use anyhow;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
struct Edge {
    from: String,
    to: String,
    modifier: i32,
}

fn parse_feels(line: &str) -> (String, String, i32) {
    let s: Vec<&str> = line.trim().split(" ").collect();

    let from = s[0];
    let to = s[10].strip_suffix(".").unwrap();

    let modifier = {
        let m = s[3].parse::<i32>().unwrap();
        if s[2] == "lose" {
            m * -1
        } else {
            m
        }
    };

    (String::from(from), String::from(to), modifier)
}


fn build_modifier_map(input: String, add_self: bool) -> HashMap<String, HashMap<String, i32>> {
    let mut m = HashMap::new();

    for (from, to, modifier) in input.lines().map(|line| parse_feels(line)) {
        if !m.contains_key(&from) {
            let inner = HashMap::new();
            m.insert(from.clone(), inner);
        }
        m.get_mut(&from).unwrap().insert(to, modifier);
    }




    if add_self {
        let me = String::from("You");
        m.insert(me.clone(), HashMap::new());
        for name in m.keys().map(|k| k.to_owned()).collect_vec() {
            m.get_mut(&me).unwrap().insert(name.clone(), 0);
            m.get_mut(&name).unwrap().insert(me.clone(), 0);
        }
    }

    m
}

fn happiness(map: &HashMap<String, HashMap<String, i32>>, seating: Vec<&String>) -> i32 {
    let mut happiness = 0;

    for (left, sub, right) in seating.iter().circular_tuple_windows() {
        happiness += map[*sub][*left];
        happiness += map[*sub][*right];
    }

    happiness
}

fn solve(input: String, add_self: bool) -> i32 {
    let guests = build_modifier_map(input, add_self);

    let names: Vec<String> = guests.keys().map(|k| k.to_owned()).collect_vec();
    let happiness = names.iter().permutations(names.len()).map(|seating| happiness(&guests, seating)).max();
    happiness.unwrap()
}


fn main() {
    let input = fetch::get_input(2015, 13);

    println!("part 1 => {}", solve(input.clone(), false));
    println!("part 2 => {}", solve(input, true));


}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_parse() {
        let expected = (String::from("a"), String::from("b"), 10);
        let line = "a would gain 10 happiness units by sitting next to b.";

        assert_eq!(parse_feels(line), expected);
    }

    #[test]
    fn p1_tests() {

        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol."#;
        let thing = build_modifier_map(input.to_string());

        let names: Vec<String> = thing.keys().map(|k| k.to_owned()).collect_vec();

        let min = names.iter().permutations(names.len()).map(|seating| happiness(&thing, seating)).max();

        assert_eq!(min, Some(330));


    }

    #[test]
    fn p2_tests() {}
}
