use std::{
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
    sync::{Mutex, MutexGuard},
};

use advent::input_store;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    IResult,
};

#[derive(Debug)]
struct Rule {
    before: String,
    after: String,
    element: String,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} -> {}", self.before, self.after, self.element)
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, before) = take(1usize)(input)?;
    let (input, after) = take(1usize)(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, element) = take(1usize)(input)?;

    Ok((
        input,
        Rule {
            before: before.to_string(),
            after: after.to_string(),
            element: element.to_string(),
        },
    ))
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        let (_, rule) = parse_rule(input).unwrap();
        rule
    }
}

struct Atom {
    element: String,
    before: Option<Rc<Mutex<Atom>>>,
    after: Option<Rc<Mutex<Atom>>>,
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let before = if let Some(b) = &self.before {
            let lock = b.lock().unwrap();
            Some(lock.element.clone())
        } else {
            None
        };

        let after = if let Some(a) = &self.after {
            let lock = a.lock().unwrap();
            Some(lock.element.clone())
        } else {
            None
        };

        f.debug_struct("Atom")
            .field("element", &self.element)
            .field("before", &before)
            .field("after", &after)
            .finish()
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut polymer = self.element.clone();

        let mut next = self.after.clone();

        while next.is_some() {
            let next_atom = next.unwrap();
            let lock = next_atom.lock().unwrap();
            polymer += &lock.element;
            next = lock.after.clone();
        }

        write!(f, "{}", polymer)
    }
}

impl Atom {
    fn new(element: String) -> Self {
        Self {
            element,
            before: None,
            after: None,
        }
    }

    fn check_rule(&self, rule: &Rule) -> bool {
        match &self.before {
            Some(before) => {
                let lock = before.lock().unwrap();
                lock.element == rule.before && self.element == rule.after
            }
            _ => false,
        }
    }

    fn insert(&mut self, rule: &Rule) {
        let before = self.before.clone();

        match before {
            Some(before) => {
                let mut before_lock = before.lock().unwrap();
                let mut new_atom = Atom::new(rule.element.clone());
                new_atom.before = self.before.clone();
                new_atom.after = before_lock.after.clone();

                let new_atom = Rc::new(Mutex::new(new_atom));
                before_lock.after = Some(new_atom.clone());
                self.before = Some(new_atom.clone());
            }
            _ => unreachable!(),
        }
    }

    fn to_vec(&self) -> Vec<Rc<Mutex<Self>>> {
        let mut next = self.after.clone();
        let mut out = Vec::new();

        while next.is_some() {
            let after = next.clone().unwrap();
            out.push(after.clone());
            let after_lock = after.lock().unwrap();
            let after_after = after_lock.after.clone();
            //out.push(after_after.clone());
            next = after_after;
        }

        out
    }

    fn count_elements(&self) -> HashMap<String, u64> {
        let mut out = HashMap::new();
        let mut next = self.after.clone();

        *out.entry(self.element.clone()).or_insert(0) += 1;

        while next.is_some() {
            let after = next.clone().unwrap();
            let after_lock = after.lock().unwrap();
            *out.entry(after_lock.element.clone()).or_insert(0) += 1;
            let after_after = after_lock.after.clone();
            //out.push(after_after.clone());
            next = after_after;
        }

        out
    }
}

fn init_polymer(given: &str) -> Vec<Rc<Mutex<Atom>>> {
    let mut out = Vec::new();
    for chr in given.trim().chars() {
        let element = chr.to_string();
        out.push(Rc::new(Mutex::new(Atom::new(element))));
    }

    for (a, b) in out.iter().tuple_windows() {
        let ac = a.clone();
        let bc = b.clone();

        if let Ok(mut a_lock) = a.lock() {
            a_lock.after = Some(bc);
        }

        if let Ok(mut b_lock) = b.lock() {
            b_lock.before = Some(ac);
        }
    }

    out
}

fn apply(first: Rc<Mutex<Atom>>, rules: &Vec<Rule>) {
    {
        let atoms = {
            let f_lock = first.lock().unwrap();
            f_lock.to_vec()
        };

        atoms.iter().for_each(|e| {
            let mut lock = e.lock().unwrap();

            for rule in rules {
                if lock.check_rule(rule) {
                    lock.insert(rule);
                    break;
                }
            }
        });
    }
}

fn main() {
    let input = input_store::get_input(2021, 14);

    //     let input = r#"NNCB

    // CH -> B
    // HH -> N
    // CB -> H
    // NH -> C
    // HB -> C
    // HC -> B
    // HN -> C
    // NN -> C
    // BH -> H
    // NC -> B
    // NB -> B
    // BN -> B
    // BB -> N
    // BC -> B
    // CC -> N
    // CN -> C"#;

    let mut input_split = input.trim().split("\n\n");

    let given = input_split.next().unwrap();
    let rules: Vec<Rule> = {
        let r = input_split.next().unwrap();
        r.lines().map(|l| l.into()).collect()
    };

    let polymer = init_polymer(given);
    // dbg!(&polymer);
    // dbg!(&rules);

    let first = { polymer.iter().take(1).next().unwrap().clone() };

    for i in 0..10 {
        println!("iteration {}", i);
        apply(first.clone(), &rules);
    }
    let element_counts = first.lock().unwrap().count_elements();

    let part_1 = element_counts.values().max().unwrap() - element_counts.values().min().unwrap();

    dbg!(element_counts);

    println!("part_1 => {}", part_1);

    // for i in 0..30 {
    //     println!("iteration {}", i + 10);
    //     apply(first.clone(), &rules);
    // }
    // let element_counts = first.lock().unwrap().count_elements();

    // let part_2 = element_counts.values().max().unwrap() - element_counts.values().min().unwrap();

    println!("part_2 => {}", "part_2");
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
