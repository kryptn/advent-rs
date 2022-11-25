use advent::input_store;
use petgraph::{
    algo::{has_path_connecting, kosaraju_scc},
    prelude::UnGraph,
    stable_graph::NodeIndex,
};

fn edges_from(input: &str) -> Vec<(u32, u32)> {
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut out = Vec::new();
    let this: u32 = input[0].parse().unwrap();

    for edge in &input[2..] {
        out.push((this, edge.trim_end_matches(",").parse().unwrap()))
    }

    out
}

fn main() {
    let input = input_store::get_input(2017, 12);

    let edges: Vec<(u32, u32)> = input
        .lines()
        .map(|line| edges_from(line.trim()))
        .flatten()
        .collect();

    let graph: UnGraph<u32, ()> = UnGraph::<u32, ()>::from_edges(edges);
    let scc = kosaraju_scc(&graph);

    let nodes_with_zero = scc
        .iter()
        .filter(|nodes| has_path_connecting(&graph, nodes[0], NodeIndex::new(0), None))
        .map(|nodes| nodes.len())
        .next()
        .unwrap();

    println!("part_1 => {}", nodes_with_zero);
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
