use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate, Coordinate3d, Direction, Space};
use petgraph::{algo::astar, data::Build, prelude::DiGraph, stable_graph::NodeIndex};

struct Blizzards {
    blizzards: Vec<(Coordinate, Direction)>,
    state: HashSet<Coordinate3d>,
    lower: Coordinate,
    upper: Coordinate,
    minute: usize,

    idx_map: HashMap<Coordinate3d, NodeIndex>,
}

impl Blizzards {
    fn new(blizzards: Vec<(Coordinate, Direction)>) -> Self {
        let space: Space<Coordinate, Direction> = blizzards
            .into_iter()
            .filter(|(_, d)| *d != Direction::None)
            .collect();
        let (lower, upper) = space.bounding_box();
        let blizzards = space.clone().into_iter().collect();

        let mut out = Self {
            blizzards,
            state: HashSet::new(),
            lower,
            upper,
            minute: 0,
            idx_map: HashMap::new(),
        };

        out.populate_state();
        out
    }

    fn start_end(&self) -> (Coordinate, Coordinate) {
        (self.lower.down(), self.upper.up())
    }

    fn wrapped_move(&self, (coord, dir): &(Coordinate, Direction)) -> (Coordinate, Direction) {
        let coord = match dir {
            Direction::Up => coord.down(),
            Direction::Down => coord.up(),
            _ => *coord + *dir,
        };

        let zerod = coord + Coordinate::from((-1, -1)) + self.upper;
        let normalized = Coordinate::from((zerod.x % self.upper.x, zerod.y % self.upper.y));
        let coord = normalized + Coordinate::from((1, 1));

        (coord, *dir)
    }

    fn populate_state(&mut self) {
        let start = self.lower.down();
        let start = (start.x, start.y, self.minute as isize).into();

        let end = self.upper.up();
        let end = (end.x, end.y, self.minute as isize).into();

        self.state.extend(&[start, end]);

        let map: HashMap<Coordinate, Direction> = self.blizzards.iter().cloned().collect();
        for coord in coordinates_within(self.lower, self.upper) {
            if !map.contains_key(&coord) {
                let state_coord = (coord.x, coord.y, self.minute as isize).into();
                self.state.insert(state_coord);
            }
        }
    }

    fn tick(&mut self) {
        self.blizzards = self
            .blizzards
            .iter()
            .map(|b| self.wrapped_move(b))
            .collect();
        self.minute += 1;
        self.populate_state();
    }

    fn full_cycle(&mut self) {
        for _ in 0..(self.upper.x * self.upper.y) {
            self.tick()
        }
    }

    fn edges(&self) -> Vec<(Coordinate3d, Coordinate3d)> {
        let cycle = self.upper.x * self.upper.y;
        let mut out = Vec::new();

        for empty in &self.state {
            let forward = empty.forward();
            let forward: Coordinate3d = (forward.x, forward.y, forward.z % cycle).into();

            let candidates = [
                forward.left(),
                forward.right(),
                forward.up(),
                forward.down(),
                forward,
            ];
            for candidate in candidates {
                if self.state.contains(&candidate) {
                    out.push((empty.clone(), candidate));
                }
            }
        }

        out
    }

    fn to_graph(&mut self) -> DiGraph<Coordinate3d, ()> {
        let mut graph = DiGraph::default();
        for node in &self.state {
            let idx = graph.add_node(node.clone());
            self.idx_map.insert(node.clone(), idx);
        }

        for (a, b) in self.edges() {
            let a_idx = self.idx_map[&a];
            let b_idx = self.idx_map[&b];

            graph.add_edge(a_idx, b_idx, ());
        }

        graph
    }
}

fn main() {
    let input = input_store::get_input(2022, 24);
    //     let input = r#"#.######
    // #>>.<^<#
    // #.<..<<#
    // #>v.><>#
    // #<^v^^>#
    // ######.#"#;

    let initial_space: Space<Coordinate, Direction> = Space::from_lines(&input);
    let pairs: Vec<(Coordinate, Direction)> = initial_space.clone().into_iter().collect();

    let mut blizzards = Blizzards::new(pairs);
    blizzards.full_cycle();

    let graph = blizzards.to_graph();

    let (start_pos, end_pos) = blizzards.start_end();
    let start = (start_pos.x, start_pos.y, 0).into();
    let start = blizzards.idx_map[&start];
    let first = astar(
        &graph,
        start,
        |idx| {
            let node = graph.node_weight(idx).unwrap();
            node.x == end_pos.x && node.y == end_pos.y
        },
        |_| 1,
        |_| 0,
    )
    .unwrap();
    println!("part_1 => {}", first.0);

    // go back to start
    let start = first.1.last().unwrap();
    let back = astar(
        &graph,
        *start,
        |idx| {
            let node = graph.node_weight(idx).unwrap();
            node.x == start_pos.x && node.y == start_pos.y
        },
        |_| 1,
        |_| 0,
    )
    .unwrap();

    // now come back
    let start = back.1.last().unwrap();
    let finish = astar(
        &graph,
        *start,
        |idx| {
            let node = graph.node_weight(idx).unwrap();
            node.x == end_pos.x && node.y == end_pos.y
        },
        |_| 1,
        |_| 0,
    )
    .unwrap();

    println!("part_2 => {}", first.0 + back.0 + finish.0);
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
