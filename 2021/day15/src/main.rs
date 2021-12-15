use std::collections::HashMap;

use advent::{
    grid::{self, bounding_box, Coordinate, Grid},
    input_store,
};

use petgraph::{
    algo::astar,
    graph::{self, DiGraph, NodeIndex, UnGraph},
    Graph,
};

fn cost(er: graph::EdgeReference<f32>) -> f32 {
    er.weight().clone()
}

fn build_graph(grid: &Grid<f32>) -> (Graph<Coordinate, f32>, NodeIndex, NodeIndex) {
    let mut coordinate_idx_lookup = HashMap::new();

    let graph = {
        let mut g = DiGraph::<Coordinate, f32>::default();

        for value in grid.keys().cloned() {
            let idx = g.add_node(value.clone());
            coordinate_idx_lookup.insert(value, idx);
        }

        for (coord, weight) in grid.iter() {
            let coord_idx = coordinate_idx_lookup.get(&coord).unwrap();

            coord
                .cardinals()
                .iter()
                .filter(|&n| grid.contains_key(n))
                .for_each(|neighbor| {
                    let neighbor_idx = coordinate_idx_lookup.get(neighbor).unwrap();
                    g.add_edge(*neighbor_idx, *coord_idx, *weight);
                });
        }

        g
    };

    let (start, end) = bounding_box(&grid);
    let start_idx = coordinate_idx_lookup.get(&start).unwrap().clone();
    let end_idx = coordinate_idx_lookup.get(&end).unwrap().clone();
    (graph, start_idx, end_idx)
}

fn main() {
    let input = input_store::get_input(2021, 15);

    //     let input = r#"1163751742
    // 1381373672
    // 2136511328
    // 3694931569
    // 7463417111
    // 1319128137
    // 1359912421
    // 3125421639
    // 1293138521
    // 2311944581"#;

    let grid: Grid<f32> = grid::from_text(&input).unwrap();

    let (graph, start, end) = build_graph(&grid);

    let (cost, path) = astar(&graph, start, |finish| finish == end, cost, |_| (0 as f32)).unwrap();

    // dbg!(coords);
    // dbg!(cost);

    println!("part_1 => {}", cost);

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
