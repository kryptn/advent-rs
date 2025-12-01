use std::{collections::HashMap, hash::Hash};

use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Direction, Space};
use colored::Colorize;
use petgraph::graph::DiGraph;

const YEAR: usize = 2023;
const DAY: usize = 23;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Square {
    Wall,

    #[default]
    Empty,
    Slope(Direction),
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            // unicode block
            '#' => Self::Wall,
            '.' => Self::Empty,
            '^' => Self::Slope(Direction::Down), // reversed because y axis is flipped
            'v' => Self::Slope(Direction::Up),   // reversed because y axis is flipped
            '<' => Self::Slope(Direction::Left),
            '>' => Self::Slope(Direction::Right),
            _ => panic!("invalid square"),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Wall => write!(f, "\u{2588}"),
            Square::Empty => write!(f, " "),
            Square::Slope(Direction::Up) => write!(f, "v"),
            Square::Slope(Direction::Down) => write!(f, "^"),
            Square::Slope(Direction::Left) => write!(f, "<"),
            Square::Slope(Direction::Right) => write!(f, ">"),
            _ => panic!("invalid square"),
        }
    }
}

struct TraversedSquare(Square, bool);

impl std::fmt::Display for TraversedSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            let value = format!("{}", self.0);
            write!(f, "{}", value.green())
        } else {
            write!(f, "{}", self.0)
        }
    }
}

fn find_longest_path(map: &Space<Coordinate, Square>) -> Vec<Coordinate> {
    let (lower, upper) = map.bounds();

    let start = map
        .iter()
        .find(|(k, s)| k.y == 0 && **s == Square::Empty)
        .unwrap()
        .0;

    let end = map
        .iter()
        .find(|(k, s)| k.y == upper.y - 1 && **s == Square::Empty)
        .unwrap()
        .0;

    let edge_map = derive_edge_map(map);

    todo!()
}

// trait IntoEdges<Idx> {
//     fn into_edges(self, idx: Idx) -> Vec<Idx>;
// }

// impl IntoEdges<(Coordinate, Direction)> for Square {

// }

// fn derive_edge_map_g<T, V>(space: &Space<Coordinate, V>) -> HashMap<T, Vec<T>> where T: Eq + Hash {
//     let mut out: HashMap<T, Vec<T>> = HashMap::new();

//     for (k, v) in space.iter() {

//     }

//     out
// }

fn derive_edge_map(
    map: &Space<Coordinate, Square>,
) -> HashMap<(Coordinate, Direction), Vec<(Coordinate, Direction)>> {
    let mut out: HashMap<(Coordinate, Direction), Vec<(Coordinate, Direction)>> = HashMap::new();

    for (k, v) in map.iter().filter(|(_, s)| **s != Square::Wall) {
        match v {
            Square::Slope(slope_dir) => {
                let this_key = (*k, *slope_dir);
                let next_k = *k + *slope_dir;
                for d in spatial::DIRECTIONS {
                    out.entry(this_key).or_default().push((next_k, d));
                }
            }
            Square::Empty => {
                for d in spatial::DIRECTIONS {
                    let candidates = &[d, d.left(), d.right()];
                    for candidate in candidates {
                        let next_k = *k + *candidate;
                        if let Some(Square::Wall) = map.get(&next_k) {
                            continue;
                        }
                        out.entry((*k, d)).or_default().push((next_k, *candidate));
                    }
                }
            }
            Square::Wall => panic!("this should be excluded"),
        }
    }

    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Node {
    position: Coordinate,
    direction: Direction,
}

fn build_graph(map: &Space<Coordinate, Square>) -> DiGraph<Coordinate, usize> {
    let mut node_idx_map = HashMap::new();
    let mut edge_idx_map = HashMap::new();
    let mut graph = DiGraph::new();

    for key in map
        .iter()
        .filter(|(_, s)| **s != Square::Wall)
        .map(|(k, _)| k)
    {
        let idx = graph.add_node(*key);
        node_idx_map.insert(*key, idx);
    }

    for (k, v) in map.iter().filter(|(_, s)| **s != Square::Wall) {
        for other in k.neighbors() {
            if let Some(Square::Wall) = map.get(&other) {
                continue;
            }

            let edge = graph.add_edge(node_idx_map[k], node_idx_map[&other], 1);
            edge_idx_map.insert((k, other), edge);
        }
    }

    graph
}

// fn consolidate_graph(graph: &mut DiGraph<Coordinate, usize>) {

//     for node in graph.n

// }

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let input = input.as_str();

    // let input = r#"#.#####################
    // #.......#########...###
    // #######.#########.#.###
    // ###.....#.>.>.###.#.###
    // ###v#####.#v#.###.#.###
    // ###.>...#.#.#.....#...#
    // ###v###.#.#.#########.#
    // ###...#.#.#.......#...#
    // #####.#.#.#######.#.###
    // #.....#.#.#.......#...#
    // #.#####.#.#.#########v#
    // #.#...#...#...###...>.#
    // #.#.#v#######v###.###v#
    // #...#.>.#...>.>.#.###.#
    // #####v#.#.###v#.#.###.#
    // #.....#...#...#.#.#...#
    // #.#########.###.#.#.###
    // #...###...#...#...#.###
    // ###.###.#.###v#####v###
    // #...#...#.#.>.>.#.>.###
    // #.###.###.#.###.#.#v###
    // #.....###...###...#...#
    // #####################.#
    // "#;

    let mut map: Space<Coordinate, Square> = Space::from(input);
    println!("{}", map);

    println!("part_1 => {}", "not done");
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
