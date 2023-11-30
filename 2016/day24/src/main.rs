use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space, Traversable};
use itertools::Itertools;

const YEAR: usize = 2016;
const DAY: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Duct {
    Wall,
    Empty,
    Visited,
    Target(usize),
}

impl std::default::Default for Duct {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Display for Duct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, " "),
            Self::Visited => write!(f, "+"),
            Self::Target(n) => write!(f, "{}", n),
        }
    }
}

impl Traversable for Duct {
    fn is_traversable(&self) -> bool {
        match self {
            Self::Wall => false,
            _ => true,
        }
    }
}

impl From<char> for Duct {
    fn from(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => Self::Target(input.to_string().parse().unwrap()),
        }
    }
}

fn print_travelled(maze: &Space<Coordinate, Duct>, path: &[Coordinate]) {
    let mut maze = maze
        .iter()
        .map(|(c, v)| (*c, *v))
        .collect::<HashMap<Coordinate, Duct>>();
    for coord in path {
        maze.entry(*coord).and_modify(|e| {
            if let Duct::Empty = e {
                *e = Duct::Visited;
            }
        });
    }
    let maze: Space<_, _> = maze.into_iter().collect();
    println!("maze => \n\n{}", maze);
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"###########
    // #0.1.....2#
    // #.#######.#
    // #4.......3#
    // ###########"#;

    let maze: Space<Coordinate, Duct> = {
        let mut temp = Vec::new();
        for (y, line) in input.trim().lines().enumerate() {
            for (x, ch) in line.trim().chars().enumerate() {
                temp.push(((x as isize, y as isize).into(), ch.into()));
            }
        }

        temp.into()
    };

    let max = maze
        .values()
        .filter_map(|v| match v {
            Duct::Target(n) => Some(*n),
            _ => None,
        })
        .max()
        .unwrap();

    let mut distances = HashMap::new();
    for start in 0..=max {
        for end in 0..=max {
            if start == end {
                continue;
            }
            let s = maze.find(|d| *d == Duct::Target(start)).unwrap();
            let e = maze.find(|d| *d == Duct::Target(end)).unwrap();
            let path = maze.a_star(&s, &e).unwrap();
            distances
                .entry(start)
                .or_insert_with(HashMap::new)
                .insert(end, path.len());
        }
    }

    let all_paths: Vec<(usize, Vec<usize>)> = (1..=max)
        .permutations(max)
        .map(|p| {
            let mut prev = 0;
            let mut total = 0;
            for n in &p {
                total += distances[&prev][n] - 1;
                prev = *n;
            }

            (total, p)
        })
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect();

    println!("part_1 => {}", all_paths.first().unwrap().0);
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
