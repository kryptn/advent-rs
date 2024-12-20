use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 20;

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    #[default]
    Wall,
    Open,
    Start,
    End,
    Cheat,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Open => write!(f, "."),
            Cell::Start => write!(f, "S"),
            Cell::End => write!(f, "E"),
            Cell::Cheat => write!(f, " "),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Open,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => unreachable!(),
        }
    }
}

fn two_steps(pos: &Coordinate) -> Vec<Coordinate> {
    pos.cardinals()
        .iter()
        .flat_map(|c| c.cardinals())
        .map(|c| c)
        .collect::<Vec<_>>()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"###############
    // #...#...#.....#
    // #.#.#.#.#.###.#
    // #S#...#.#.#...#
    // #######.#.#.###
    // #######.#.#...#
    // #######.#.###.#
    // ###..E#...#...#
    // ###.#######.###
    // #...###...#...#
    // #.#####.#.###.#
    // #.#...#.#.#...#
    // #.#.#.#.#.#.###
    // #...#...#...###
    // ###############
    // "#;

    let maze: Space<Coordinate, Cell> = input.into();

    let cheats: HashSet<(Coordinate, Coordinate)> = maze
        .keys()
        .map(|c| repeat(*c).zip(two_steps(c)))
        .flatten()
        .filter(|(a, b)| {
            a != b
                && matches!(
                    maze.get(a),
                    Some(Cell::Open) | Some(Cell::End) | Some(Cell::Start)
                )
                && matches!(
                    maze.get(b),
                    Some(Cell::Open) | Some(Cell::End) | Some(Cell::Start)
                )
        })
        .collect();

    let edges = |pos: &Coordinate| -> Vec<Coordinate> {
        pos.cardinals()
            .iter()
            .filter(|c| matches!(maze.get(c), Some(Cell::Open) | Some(Cell::End)))
            .map(|c| *c)
            .collect::<Vec<_>>()
    };

    let is_goal = |c: &Coordinate| -> bool { matches![maze.get(c), Some(Cell::End)] };
    let cost = |_: &Coordinate| -> Option<usize> { Some(1) };

    let start = maze
        .iter()
        .find(|(_, v)| **v == Cell::Start)
        .map(|(k, _)| *k)
        .unwrap();

    let result = advent_toolbox::algo::dijkstra(&[start], edges, is_goal, Some(cost));

    let mut part_1 = 0;

    let mut costs: HashMap<usize, usize> = HashMap::new();

    for cheat in cheats {
        let (a, b) = cheat;

        let a_cost = result.costs.get(&a).unwrap();
        let b_cost = result.costs.get(&b).unwrap();

        if a_cost > b_cost {
            continue;
        }

        let savings = b_cost - a_cost - 2;
        // println!("({}, {}) -> ({}, {})  = {}", a, a_cost, b, b_cost, savings);
        costs.entry(savings).and_modify(|c| *c += 1).or_insert(1);
        if savings >= 100 && true {
            part_1 += 1;
        }
    }

    for key in costs.keys().sorted_by(|a, b| a.cmp(b)) {
        println!("{} => {}", key, costs[key]);
    }

    // dbg!(costs);

    println!("part_1 => {}", part_1);
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
