use std::collections::HashMap;

use advent::{
    grid::{self, bounding_box, coordinates_within, manhattan, Coordinate, Grid},
    input_store,
};

use petgraph::{
    algo::astar,
    graph::{DiGraph, NodeIndex},
    Graph,
};

fn build_graph(grid: &Grid<i32>) -> (Graph<Coordinate, i32>, NodeIndex, NodeIndex) {
    let mut coordinate_idx_lookup = HashMap::new();

    let graph = {
        let mut g = DiGraph::<Coordinate, i32>::default();

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

fn explore_grid(source: &Grid<i32>, by: Coordinate) -> Grid<i32> {
    let mut out = Grid::new();
    let (_, size) = bounding_box(&source);
    let size = size + (1, 1).into();

    for sector in coordinates_within((0, 0).into(), by) {
        for (coord, risk) in source {
            let shifted = coord.clone() + (sector.x * size.x, sector.y * size.y).into();

            let risk = {
                let mut risk = *risk as i32;
                risk += manhattan((0, 0).into(), sector);
                risk -= 1;
                risk = risk % 9;
                risk + 1
            };

            out.insert(shifted, risk as i32);
        }
    }

    out
}

fn main() {
    let input = input_store::get_input(2021, 15);

    let part_1 = {
        let grid: Grid<i32> = grid::from_text(&input).unwrap();
        let (graph, start, end) = build_graph(&grid);
        let (cost, _) = astar(
            &graph,
            start,
            |finish| finish == end,
            |e| e.weight().clone(),
            |_| 0,
        )
        .unwrap();
        println!(
            "graph nodes: {}, graph edges: {}",
            graph.node_count(),
            graph.edge_count()
        );
        cost
    };

    println!("part_1 => {}", part_1);

    let part_2 = {
        let grid: Grid<i32> = grid::from_text(&input).unwrap();
        let explored_grid = explore_grid(&grid, (4, 4).into());
        let (graph, start, end) = build_graph(&explored_grid);
        let (cost, _) = astar(
            &graph,
            start,
            |finish| finish == end,
            |e| e.weight().clone(),
            |_| 0,
        )
        .unwrap();

        println!(
            "graph nodes: {}, graph edges: {}",
            graph.node_count(),
            graph.edge_count()
        );

        cost
    };

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
