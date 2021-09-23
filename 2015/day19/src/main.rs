use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    fmt,
    hash::{Hash, Hasher},
    io::{self, Write},
    pin::Pin,
    str::FromStr,
};

use advent::{
    fetch,
    grid::{self, Coordinate},
};
use anyhow;
use itertools::Itertools;
use regex::{Regex, RegexSet};

#[derive(Debug)]
struct Structure<'a> {
    replacements: &'a Vec<(&'a str, &'a str)>,
    original: String,

    prefix: String,
    seen: HashSet<String>,
    total: u32,
}

impl<'a> Structure<'a> {
    fn new(replacements: &'a Vec<(&'a str, &'a str)>, prefix: String, original: String) -> Self {
        Self {
            replacements,
            original,
            prefix,
            seen: HashSet::new(),
            total: 0,
        }
    }

    fn all(&self, c: char) -> bool {
        self.prefix.chars().all(|ch| ch == c ) && self.original.chars().all(|ch| ch == c )
    }
}

fn replace(s: Structure) -> Vec<String> {
    let mut s = s;

    let mut out = Vec::new();

    for (i, (matcher, repl)) in s.replacements.iter().enumerate() {
        let next_structure = {
            if s.original.starts_with(matcher) {
                let mut next_prefix = s.prefix.clone();
                //next_prefix.push_str(repl);
                next_prefix.push_str(&s.original[0..1]);

                let mut new = s.prefix.clone();
                new.push_str(repl);

                let next_original = &s.original[matcher.len()..s.original.len()];
                new.push_str(next_original);

                s.seen.insert(new.clone());
                out.push(new);

                let next = Structure::new(&s.replacements, next_prefix, next_original.to_string());

                //println!("matched on {} and replaced with {}. \n\tprefix: {}\n\tsuffix: {}\n\n", matcher, repl, s.prefix, s.original);

                next
            } else {
                let mut next_prefix = s.prefix.clone();
                next_prefix.push_str(&s.original[0..1]);

                let next_original = &s.original[1..s.original.len()];

                let next = Structure::new(&s.replacements, next_prefix, next_original.to_string());

                //println!("did not match {}. \n\tprefix: {}\n\tsuffix: {}\n\n", matcher, s.prefix, s.original);

                next
            }
        };

        //println!("next struture: \n\tprefix: {}\n\tsuffix: {}\n\n", &next_structure.prefix, &next_structure.original);

        if next_structure.original.len() > 0 && i == 0 {
            for new_str in replace(next_structure) {
                out.push(new_str);
            }
        }
    }

    out
}

fn reduce(s: Structure, target: &String) -> Option<i32> {

    //println!("next struture: \n\tprefix: {}\n\tsuffix: {}\n\n", &s.prefix, &s.original);

    if s.original == *target && s.prefix.len() == 0 {
        return Some(1);
    } else if s.original.len() == 0 {
        return None;
    } else if s.all(target.chars().next().unwrap()) {
        return None
    }

    let branches: Vec<i32> = s.replacements.iter().map(|(original, expanded)| {
        if s.original.starts_with(expanded) && (original != target || s.prefix.len() == 0) {
            let mut next = s.prefix.clone();
            next.push_str(original);
            next.push_str(&s.original[expanded.len()..s.original.len()]);
            Structure::new(&s.replacements, String::new(), next)
        } else {
            let mut next_prefix = s.prefix.clone();
            next_prefix.push_str(&s.original[0..1]);

            let next_original = &s.original[1..s.original.len()];
            Structure::new(&s.replacements, next_prefix, next_original.to_string())
        }
    }).map(|next| reduce(next, target)).filter(|c| *c != None).map(|o| o.unwrap()).collect();


    if branches.len() == 0 {
        return None;
    }

    Some(branches.iter().min().unwrap() + 1)
}

fn main() {
    let input = fetch::get_input(2015, 19);

//     let input = r#"H => HO
// H => OH
// O => HH

// HOH"#;

     let input = r#"e => O
e => H
H => HO
H => OH
O => HH

HOHOHO"#;

    let mut replacements: Vec<(&str, &str)> = Vec::new();
    let lines = input.lines().map(|l| l.trim()).collect_vec();
    let starting = lines[lines.len() - 1];
    for line in lines {
        if line.len() > 0 {
            let parts = line.split(" ").collect_vec();
            replacements.push((parts[0], parts[2]))
        } else {
            break;
        }
    }

    dbg!(&replacements);
    dbg!(&starting);

    let s = Structure::new(&replacements, "".to_string(), starting.to_string());


    let all_replacements = replace(s);

    let mut repl_set = HashSet::new();
    for repl in &all_replacements{
        repl_set.insert(repl);
    }

    //dbg!(&all_replacements);
    //dbg!(&repl_set);

    println!("part 1 -> {}", repl_set.len());

    let mut r = replacements.clone();
    r.reverse();

    let s = Structure::new(&r, "".to_string(), starting.to_string());

    println!("part 2 => {:?}", reduce(s, &String::from("e")));

}

#[cfg(test)]
mod test {

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
