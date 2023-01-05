use std::{collections::HashMap, u32};

use advent::input_store;

use advent_toolbox::spatial::{Coordinate, Space, Traversable};
use lazy_static::lazy_static;
use petgraph::{algo::astar, prelude::DiGraph, stable_graph::node_index};

lazy_static! {
    static ref HEIGHT_MAP: HashMap<char, u32> = {
        let mut map: HashMap<char, u32> = ('a'..='z')
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i as u32))
            .collect();
        map.insert('S', 0);
        map.insert('E', 25);
        map
    };
    static ref LETTER_HEIGHT_MAP: HashMap<u32, char> = {
        let map: HashMap<u32, char> = ('a'..='z')
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as u32, c))
            .collect();
        map
    };
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Height(u32);

impl From<char> for Height {
    fn from(ch: char) -> Self {
        Self(HEIGHT_MAP[&ch])
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LETTER_HEIGHT_MAP[&self.0])
    }
}

fn valid_step(a: u32, b: u32) -> bool {
    let a = a as i32;
    let b = b as i32;
    b - a <= 1
}

struct Terrain {
    terrain: Space<Coordinate, Height>,
    start: Coordinate,
    end: Coordinate,

    extent: Coordinate,
}

impl Traversable<Coordinate> for Terrain {
    fn connected(&self, _start: &Coordinate, _end: &Coordinate) -> bool {
        todo!()
    }
}

impl From<&str> for Terrain {
    fn from(input: &str) -> Self {
        let mut terrain: Space<Coordinate, Height> = input.into();

        let mut start: Option<Coordinate> = None;
        let mut end: Option<Coordinate> = None;

        for (y, line) in input.trim().lines().enumerate() {
            for (x, value) in line.trim().chars().enumerate() {
                let c = (x, y).into();
                match value {
                    'S' => start = Some(c),
                    'E' => end = Some(c),
                    _ => {}
                }
                let vheight = value.into();
                // println!("{:?}, {:?}, {:?}", c, &value, &vheight);
                terrain.insert(c, vheight);
            }
        }
        let start = start.expect("given in map");
        let end = end.expect("given in map");

        let (_, extent) = terrain.bounding_box();

        // println!("start: {}, end: {}", start, end);

        Self {
            terrain,
            start,
            end,
            extent,
        }
    }
}

impl Terrain {
    fn traversable_edges(&self) -> Vec<(u32, u32)> {
        let _width = (self.extent.x + 1) as u32;

        self.terrain
            .iter()
            .map(|(coord, height)| {
                let mut e = Vec::new();
                for neighbor in coord.cardinals() {
                    let v = if let Some(neighbor_height) = self.terrain.get(&neighbor) {
                        // let height_diff =
                        //     test_height(height.0 as isize, neighbor_height.0 as isize);
                        // println!(
                        //     "{} ({}): {}, {} ({}): {}, delta: {}",
                        //     coord,
                        //     self.to_idx(*coord),
                        //     height,
                        //     neighbor,
                        //     self.to_idx(neighbor),
                        //     neighbor_height,
                        //     height_diff
                        // );

                        if valid_step(height.0, neighbor_height.0) {
                            let left = self.to_idx(*coord);
                            let right = self.to_idx(neighbor);

                            Some((left, right))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    e.push(v)
                }
                e
            })
            .flatten()
            .filter_map(|item| item)
            .collect()
    }

    fn to_idx(&self, c: Coordinate) -> u32 {
        let v = (c.y * (self.extent.x + 1)) + c.x;
        v as u32
    }

    fn from_idx(&self, idx: u32) -> Coordinate {
        let x = idx as isize % self.extent.x;
        let y = idx as isize / self.extent.x;
        (x, y).into()
    }

    fn shortest_distance(&self, start: Coordinate, end: Coordinate) -> Option<usize> {
        let edges = self.traversable_edges();

        let graph: DiGraph<u32, ()> = DiGraph::from_edges(edges);
        let start_node = node_index(self.to_idx(start) as usize);
        let end_node = node_index(self.to_idx(end) as usize);

        let path = astar(&graph, start_node, |f| f == end_node, |_| 1, |_| 1);

        // let path = path.unwrap().1;

        match path {
            Some(path) => Some(path.1.len() - 1),
            None => None,
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 12);
    // let input = r#"Sabqponm
    // abcryxxl
    // accszExk
    // acctuvwj
    // abdefghi"#
    //     .to_string();

    let terrain: Terrain = input.as_str().into();
    let part_1 = terrain
        .shortest_distance(terrain.start, terrain.end)
        .unwrap();
    println!("part_1 => {}", part_1);

    let part_2 = terrain
        .terrain
        .iter()
        .filter(|(c, h)| h.0 == 0 && (c.x == 0 || c.x == 2))
        .map(|(&c, _)| terrain.shortest_distance(c, terrain.end))
        .filter_map(|v| v)
        .min()
        .unwrap();
    println!("part_2 => {}", part_2);
}

#[cfg(test)]
mod test {

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
