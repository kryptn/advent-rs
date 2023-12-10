use std::{
    collections::{HashMap, VecDeque},
    default,
};

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate, Space};
use colored::Colorize;
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Segment {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
    original: char,
    repr: char,
    traversed: bool,
    is_network: bool,
    origin: bool,
    center: bool,
}

impl Segment {
    fn valid_paths(&self, from: Coordinate) -> impl Iterator<Item = Coordinate> {
        let mut out = Vec::new();
        if self.up {
            out.push(from.down());
        }
        if self.right {
            out.push(from.right());
        }
        if self.down {
            out.push(from.up());
        }
        if self.left {
            out.push(from.left());
        }
        out.into_iter()
    }

    fn expand(&self) -> HashMap<Coordinate, Self> {
        let mut out = HashMap::new();
        if self.up {
            out.insert(Coordinate::new(0, -1), Self::from('|'));
        }
        if self.right {
            out.insert(Coordinate::new(1, 0), Self::from('-'));
        }
        if self.down {
            out.insert(Coordinate::new(0, 1), Self::from('|'));
        }
        if self.left {
            out.insert(Coordinate::new(-1, 0), Self::from('-'));
        }
        let mut center = Self::from(self.original);
        center.center = true;

        out.insert(Coordinate::new(0, 0), center);

        for (_, segment) in out.iter_mut() {
            segment.traversed = self.traversed;
            segment.is_network = self.is_network;
        }

        for n in Coordinate::new(0, 0).neighbors() {
            if !out.contains_key(&n) {
                out.insert(n, Self::from('.'));
            }
        }

        out
    }
}

impl default::Default for Segment {
    fn default() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
            original: '.',
            repr: '.',
            traversed: false,
            is_network: false,
            origin: false,
            center: false,
        }
    }
}

impl From<char> for Segment {
    fn from(value: char) -> Self {
        let mut states = [false, false, false, false];
        let mut repr = ' ';
        match value {
            '|' => {
                states[0] = true;
                states[2] = true;

                repr = '║';
            }
            '-' => {
                states[1] = true;
                states[3] = true;
                repr = '═';
            }
            'L' => {
                states[0] = true;
                states[1] = true;
                repr = '╚'
            }
            'J' => {
                states[0] = true;
                states[3] = true;
                repr = '╝';
            }
            '7' => {
                states[2] = true;
                states[3] = true;
                repr = '╗';
            }
            'F' => {
                states[1] = true;
                states[2] = true;
                repr = '╔';
            }
            '.' => {
                repr = ' ';
            }
            'S' => {
                states[0] = false;
                states[1] = false;
                states[2] = false;
                states[3] = false;
                repr = 'S';
            }

            _ => panic!("unknown segment"),
        }
        Self {
            up: states[0],
            right: states[1],
            down: states[2],
            left: states[3],
            original: value,
            repr,
            traversed: false,
            is_network: false,
            origin: false,
            center: false,
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.origin, self.traversed, self.is_network) {
            (true, _, _) => write!(f, "{}", self.repr.to_string().red()),
            (false, true, true) => write!(f, "{}", self.repr.to_string().blue()),
            (false, false, _) => write!(f, "{}", self.repr.to_string()),
            (false, true, false) => write!(f, "{}", self.repr.to_string().yellow()),
        }
    }
}

struct Network {
    space: Space<Coordinate, Segment>,
    distance_map: HashMap<Coordinate, usize>,
}

impl Network {
    fn new(space: Space<Coordinate, Segment>) -> Self {
        let mut distance_map = HashMap::new();
        let animal_start = space.find(|s| s.original == 'S').unwrap();
        distance_map.insert(animal_start, 0);

        let animal_actual = {
            let mut possible = [false, false, false, false];
            if let Some(s) = space.get(&animal_start.up()) {
                possible[0] = s.down;
            }
            if let Some(s) = space.get(&animal_start.right()) {
                possible[1] = s.left;
            }
            if let Some(s) = space.get(&animal_start.down()) {
                possible[2] = s.up;
            }
            if let Some(s) = space.get(&animal_start.left()) {
                possible[3] = s.right;
            }
            Segment {
                up: possible[0],
                right: possible[1],
                down: possible[2],
                left: possible[3],
                original: 'S',
                repr: 'S',
                traversed: false,
                is_network: false,
                origin: false,
                center: false,
            }
        };

        let mut out = Self {
            space,
            distance_map,
        };
        out.space.insert(animal_start, animal_actual);
        out.space
            .entry((0, 0).into())
            .and_modify(|s| s.origin = true);

        out
    }

    fn traverse(&mut self, from: Coordinate) {
        let mut queue: VecDeque<_> = vec![from].into_iter().collect();

        while let Some(coord) = queue.pop_front() {
            println!("{}", coord);
            let this = self.space.get(&coord).unwrap().clone();
            let this_distance = self.distance_map.get(&coord).unwrap().clone();
            let valid_paths: Vec<_> = this
                .valid_paths(coord)
                .filter(|c| !self.distance_map.contains_key(c) && self.space.contains_key(c))
                .collect();

            for valid in valid_paths {
                self.distance_map.insert(valid, this_distance + 1);
                queue.push_back(valid);
            }

            self.space.insert(
                coord,
                Segment {
                    traversed: true,
                    is_network: true,
                    ..this
                },
            );

            // println!("\n\n\n{}", self.space)
        }
    }

    fn traverse_outer(&mut self) {
        let (lower, upper) = self.space.bounding_box();
        dbg!(&lower, &upper);
        let lower = (lower.x - 1, lower.y - 1).into();
        let upper = (upper.x + 1, upper.y + 1).into();
        let mut queue: VecDeque<_> = coordinates_within(lower, upper)
            .into_iter()
            .filter(|c| !self.space.contains_key(c))
            .collect();

        while let Some(candidate) = queue.pop_front() {
            // println!("{}", queue.len());
            if let Some(s) = self.space.get(&candidate) {
                if s.is_network || s.traversed {
                    continue;
                }
            }

            for cd in candidate.cardinals() {
                if let Some(s) = self.space.get(&cd) {
                    if !s.is_network {
                        queue.push_back(cd);
                    }
                }
            }

            self.space
                .entry(candidate)
                .and_modify(|s| s.traversed = true);

            // println!("\n\n\n{}", self.space)
        }
    }

    fn expand(&self) -> Self {
        let out = self
            .space
            .iter()
            .map(|(coord, segment)| {
                let mut out = Vec::new();
                let offset: Coordinate = (coord.x * 3, coord.y * 3).into();
                for (delta, segment) in segment.expand() {
                    out.push((offset + delta, segment));
                }
                out
            })
            .flatten()
            .collect();

        Self {
            space: out,
            distance_map: self.distance_map.clone(),
        }
    }

    fn collapse(&self) -> Self {
        let out = self
            .space
            .iter()
            .filter(|(_, segment)| segment.center)
            .map(|(coord, segment)| {
                let coordinate = Coordinate::new(coord.x / 3, coord.y / 3);
                (coordinate, segment.clone())
            })
            .collect();

        Self {
            space: out,
            distance_map: self.distance_map.clone(),
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#".F----7F7F7F7F-7....
    // .|F--7||||||||FJ....
    // .||.FJ||||||||L7....
    // FJL7L7LJLJ||LJ.L-7..
    // L--J.L7...LJS7F-7L7.
    // ....F-J..F7FJ|L7L7L7
    // ....L7.F7||L7|.L7L7|
    // .....|FJLJ|FJ|F7|.LJ
    // ....FJL-7.||.||||...
    // ....L---J.LJ.LJLJ..."#
    //     .to_string();

    // let input = input.trim().lines().map(|l| l.trim()).rev().join("\n");
    let input = input.as_str();
    let space: Space<Coordinate, Segment> = Space::from(input);
    let mut network = Network::new(space);

    let animal = network.space.find(|s| s.original == 'S').unwrap();
    // println!("{}", network.space);
    network.traverse(animal);

    // dbg!(&network.distance_map);

    let farthest = network.distance_map.values().max().unwrap();

    println!("{}", network.space);
    println!("part_1 => {}", farthest);

    let mut new_network = network.expand();
    new_network.traverse_outer();
    println!("{}", new_network.space);

    let new_new = new_network.collapse();
    println!("{}", new_new.space);

    let inner = new_new.space.values().filter(|s| !s.traversed).count();
    println!("part_2 => {}", inner);
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
