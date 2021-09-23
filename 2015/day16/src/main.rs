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

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum PersonalityTrait {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FromStr for PersonalityTrait {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_suffix(":").unwrap_or(s);

        match s {
            "children" => Ok(Self::Children),
            "cats" => Ok(Self::Cats),
            "samoyeds" => Ok(Self::Samoyeds),
            "pomeranians" => Ok(Self::Pomeranians),
            "akitas" => Ok(Self::Akitas),
            "vizslas" => Ok(Self::Vizslas),
            "goldfish" => Ok(Self::Goldfish),
            "trees" => Ok(Self::Trees),
            "cars" => Ok(Self::Cars),
            "perfumes" => Ok(Self::Perfumes),
            _ => Err(anyhow::anyhow!("parse error")),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Aunt {
    num: i32,
    traits: HashMap<PersonalityTrait, i32>,
}

impl FromStr for Aunt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect_vec();

        let mut traits = HashMap::new();

        for i in (2..parts.len()).step_by(2) {
            let t = PersonalityTrait::from_str(parts[i]).unwrap();
            let n = parts[i + 1]
                .strip_suffix(",")
                .unwrap_or(parts[i + 1])
                .parse::<i32>()
                .unwrap();
            traits.insert(t, n);
        }

        Ok(Aunt {
            num: parts[1].strip_suffix(":").unwrap().parse::<i32>().unwrap(),
            traits,
        })
    }
}

impl Aunt {
    fn like(&self, other: &Self) -> bool {
        for (key, value) in &self.traits {
            if !other.traits.contains_key(&key) {
                return false;
            }

            if let Some(val) = other.traits.get(&key) {
                if value != val {
                    return false;
                }
            }
        }

        true
    }

    fn actually_like(&self, other: &Self) -> bool {
        for (key, value) in &self.traits {
            if !other.traits.contains_key(&key) {
                return false;
            }

            if let Some(val) = other.traits.get(&key) {
                match key {
                    PersonalityTrait::Cats | PersonalityTrait::Trees => {
                        if value <= val {
                            return false;
                        }
                    }
                    PersonalityTrait::Pomeranians | PersonalityTrait::Goldfish => {
                        if value >= val {
                            return false;
                        }
                    }
                    _ => {
                        if value != val {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let input = fetch::get_input(2015, 16);

    let expected = Aunt::from_str(EXPECTED).unwrap();

    let aunts = input
        .lines()
        .map(|line| Aunt::from_str(line).unwrap())
        .collect_vec();

    let matches = aunts.iter().filter(|a| a.like(&expected)).collect_vec();
    dbg!(matches);

    let matches = aunts
        .iter()
        .filter(|a| a.actually_like(&expected))
        .collect_vec();
    dbg!(matches);
}

const EXPECTED: &str = "Sue 000: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_like() {
        let fits = Aunt::from_str("Sue 000: cats: 7, akitas: 0, vizslas: 0, cars: 2, perfumes: 1")
            .unwrap();
        let expected = Aunt::from_str(EXPECTED).unwrap();

        dbg!(&fits);
        dbg!(&expected);

        assert_eq!(fits.like(&expected), true);
        assert_eq!(expected.like(&fits), false);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
