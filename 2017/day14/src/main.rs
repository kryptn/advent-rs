use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::hashers::knot::{self, KnotHasher};
use advent_toolbox::utils::{neighbors, adjacent};
use petgraph::algo::{kosaraju_scc, connected_components};
use petgraph::prelude::DiGraph;
use petgraph::{prelude::UnGraph, stable_graph::NodeIndex};

fn normalize((x, y): &(u32, u32)) -> u32 {
    let r = (y * 128) + x;
    r as u32
}

fn denormalize(idx: u32) -> (u32, u32) {
    let y = idx / 128;
    let rem = idx - (y * 128);
    return (rem, y)
}

fn settle((a, b): (u32, u32)) -> (u32, u32) {
    if a >= b {
        (b, a)
    } else {
        (a, b)
    }
}



fn main() {
    let input = input_store::get_input(2017, 14).trim().to_string();
    let input = "flqrgnkx";

    let knot_hashes: Vec<KnotHasher> = (0..128)
        .into_iter()
        .map(|row| format!("{}-{}", &input, row))
        .map(|seed| {
            // println!("knot seed: {}", seed);
            let mut knot = KnotHasher::new_from_str(256, &seed);
            knot.round();
            knot
        })
        .collect();

    let squares: usize = knot_hashes
        .iter()
        .map(|kh| kh.as_bin_str().chars().filter(|&ch| ch == '1').count())
        .sum();

    println!("part_1 => {}", squares);

    let grid: HashMap<(u32, u32), bool> = knot_hashes
        .iter()
        .enumerate()
        .map(|(y, kh)| {
            kh.as_bin_str()
                .chars()
                .enumerate()
                .map(|(x, ch)| ((x as u32, y as u32), ch == '1'))
                .collect::<Vec<((u32, u32), bool)>>()
        })
        .flatten()
        .collect();

    let mut edges: HashSet<(u32, u32)> = HashSet::new();

    for cell in grid.keys().cloned() {
        let lhs = grid.get(&cell).unwrap_or(&false);
        for neighbor in adjacent(cell) {
            let rhs = grid.get(&neighbor).unwrap_or(&false);
            if *lhs && *rhs {
                // let edge = settle((normalize(&cell), normalize(&neighbor)));
                let edge = (normalize(&cell), normalize(&neighbor));
                edges.insert(edge);
            }
        }
    }

    // let edges = vec!((1, 2), (2, 3), (3, 4));

    println!("{} edges", edges.len());

    let graph = DiGraph::<u32, usize>::from_edges(&edges);
    let scc = connected_components(&graph);

    // dbg!(&graph);

    dbg!(scc);

    println!("part_2 => {}", scc);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn verify_unique() {
        let mut nodes = HashSet::new();
        let length = 64;
        for y in 0..length {
            for x in 0..length {
                let normalized = normalize(&(x, y));
                nodes.insert(normalized);
            }
        }

        assert_eq!(nodes.len(), (length * length) as usize)
    }

    #[rstest]
    #[case(1, 2)]
    #[case(20, 30)]
    #[case(120, 110)]

    fn verify_normalize(#[case] x: u32, #[case] y: u32) {
        let start = (x, y);
        let normalized = normalize(&start);
        assert_eq!(start, denormalize(normalized))
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
