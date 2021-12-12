use core::hash::Hash;
use std::{collections::HashMap, fmt::Debug, rc::Rc, sync::Mutex};

use advent::input_store;


struct Cave {
    name: String,
    connections: Mutex<Vec<Rc<Cave>>>,
}

impl Cave {
    fn is_big(&self) -> bool {
        self.name.chars().all(|c| ('A'..='Z').contains(&c))
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Cave {}

impl Hash for Cave {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lock = self.connections.lock().unwrap();
        let connections: Vec<String> = lock.iter().map(|c| c.name.clone()).collect();
        f.debug_struct("Cave")
            .field("name", &self.name)
            .field("connections", &connections)
            .finish()
    }
}

impl From<String> for Cave {
    fn from(name: String) -> Self {
        Self {
            name,
            connections: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Clone, Debug)]
struct Visitor {
    path: Vec<Rc<Cave>>,
    destination: String,
}

impl Visitor {
    fn new(start: Rc<Cave>, destination: String) -> Self {
        let path = vec![start];
        Self { path, destination }
    }

    fn with_path(self, path: Rc<Cave>) -> Self {
        let mut this = self;
        this.path.push(path);
        this
    }

    fn step(&self) -> Vec<Self> {
        let current = self.path.get(self.path.len() - 1).unwrap();

        let valid_connections: Vec<Rc<Cave>> = {
            let lock = current.connections.lock().unwrap();
            lock.iter()
                .cloned()
                .filter(|c| c.is_big() || !self.path.contains(c))
                .collect()
        };

        valid_connections
            .iter()
            .cloned()
            .map(|c| {
                let next = self.clone();
                next.with_path(c)
            })
            .collect()
    }

    fn at_destination(&self) -> bool {
        let current = self.path.get(self.path.len() - 1).unwrap();
        current.name == self.destination
    }
}

fn part_1(caves: &HashMap<String, Rc<Cave>>) -> Vec<Visitor> {
    let mut out: Vec<Visitor> = Vec::new();
    let start = "start".to_string();
    let start = caves.get(&start).unwrap().clone();

    let initial = Visitor::new(start, "end".to_string());

    let mut before = vec![initial];

    while !before.is_empty() {
        let after: Vec<Visitor> = before.iter().map(|v| v.step()).flatten().collect();
        before = Vec::new();
        for visitor in after {
            if visitor.at_destination() {
                out.push(visitor);
            } else {
                before.push(visitor);
            }
        }
    }

    out
}

fn main() {
    let input = input_store::get_input(2021, 12);

    //     let input = r#"start-A
    // start-b
    // A-c
    // A-b
    // b-d
    // A-end
    // b-end"#;

    let mut caves: HashMap<String, Rc<Cave>> = HashMap::new();
    let mut edges = Vec::new();

    for line in input.trim().lines() {
        let mut edge = line.trim().split("-");

        let left = {
            let left = edge.next().unwrap().to_string();
            if !caves.contains_key(&left) {
                caves.insert(left.clone(), Rc::new(left.clone().into()));
            }

            caves.get(&left).unwrap().clone()
        };

        let right = {
            let right = edge.next().unwrap().to_string();
            if !caves.contains_key(&right) {
                caves.insert(right.clone(), Rc::new(right.clone().into()));
            }

            caves.get(&right).unwrap().clone()
        };

        edges.push((left, right));
    }

    for (left, right) in edges {
        let mut left_lock = left.connections.lock().unwrap();
        left_lock.push(right.clone());

        let mut right_lock = right.connections.lock().unwrap();
        right_lock.push(left.clone());
    }

    let visitors = part_1(&caves);

    println!("part_1 => {}", visitors.len());
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
