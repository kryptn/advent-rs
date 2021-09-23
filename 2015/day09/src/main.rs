use std::{collections::{HashMap, HashSet, VecDeque}, convert::TryInto, hash::{Hash, Hasher}, str::FromStr};

use advent::fetch;
use anyhow;
use itertools::Itertools;


#[derive(Hash, PartialOrd, PartialEq, Eq, Debug)]
struct Edge {
    a: String,
    b: String,
}

impl Edge {
    pub fn new(a: String, b: String) -> Self {
        if a > b {
            Self{a: b, b: a}
        } else {
            Self {a, b}
        }

    }
}



fn main() {
    let input = fetch::get_input(2015, 9);
    let mut map: HashMap<Edge, i32> = HashMap::new();
    let mut cities:  HashSet<String> = HashSet::new();

    for line in input.lines() {
        dbg!(line);
        let line: Vec<&str> = line.trim().split(" ").into_iter().collect();
        let a = line[0].to_string();
        let b = line[2].to_string();

        cities.insert(a.clone());
        cities.insert(b.clone());

        let dist = line[4].parse::<i32>().unwrap();

        map.insert(Edge::new(a, b), dist);
    }

    // dbg!(&map);
    // dbg!(&cities);

    let mut longest = 0;
    let mut shortest = -1;

    for c in cities.iter().permutations(cities.len()) {
        let v: i32 = c.iter().tuple_windows().map(|(a, b)| {
            let a = a.clone().to_owned();
            let b = b.clone().to_owned();
            map.get(&Edge::new(a, b)).unwrap()
        }).sum();

        if shortest < 0 || v < shortest {

            shortest = v;
        }

        if v > longest {
            longest = v;
        }
    }

    println!("part 1 => {}", shortest);
    println!("part 2 => {}", longest);




}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {

    }

    #[test]
    fn p2_tests() {

    }
}
