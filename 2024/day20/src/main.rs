use std::{collections::HashSet, iter::repeat};

use advent::input_store;
use advent_toolbox::{
    algo::DijkstraResult,
    spatial::{coordinates_within, Coordinate, Space},
};

const YEAR: usize = 2024;
const DAY: usize = 20;

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    #[default]
    Wall,
    Open,
    Start,
    End,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Open => write!(f, "."),
            Cell::Start => write!(f, "S"),
            Cell::End => write!(f, "E"),
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

fn n_steps(pos: &Coordinate, n: usize) -> Vec<Coordinate> {
    let outer = Coordinate::new(n as isize, n as isize);
    let lower: Coordinate = *pos - outer;
    let upper: Coordinate = *pos + outer;

    coordinates_within(lower, upper)
        .iter()
        .filter(|c| c.distance(pos) <= n)
        .map(|c| *c)
        .collect()
}

fn solve_part(
    maze: &Space<Coordinate, Cell>,
    result: &DijkstraResult<Coordinate>,
    n: usize,
    savings_by: usize,
) -> usize {
    let cheats: HashSet<(Coordinate, Coordinate)> = maze
        .keys()
        .map(|c| repeat(*c).zip(n_steps(c, n)))
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

    let mut out = 0;

    for cheat in cheats {
        let (a, b) = cheat;

        let a_cost = result.costs.get(&a).unwrap();
        let b_cost = result.costs.get(&b).unwrap();

        if a_cost > b_cost {
            continue;
        }

        let savings = b_cost - a_cost - b.distance(&a);
        if savings >= savings_by {
            out += 1;
        }
    }

    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"###############
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

    let part_1 = solve_part(&maze, &result, 2, 100);

    println!("part_1 => {}", part_1);

    let part_2 = solve_part(&maze, &result, 20, 100);

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
