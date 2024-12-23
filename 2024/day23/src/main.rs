use std::collections::HashSet;

use advent::input_store;
use itertools::Itertools;
use petgraph::{graph::UnGraph, prelude::UnGraphMap};

const YEAR: usize = 2024;
const DAY: usize = 23;

fn search<'a>(graph: &UnGraphMap<&'a str, i32>, start: &'a str, k: usize) -> Vec<Vec<&'a str>> {
    if k == 0 {
        return vec![vec![start]];
    }

    let mut paths = vec![];
    for node in graph.neighbors(start) {
        let lower_paths = search(graph, node, k - 1);

        for path in lower_paths {
            let mut p = vec![start];
            p.extend(path);
            paths.push(p);
        }
    }

    paths
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";

    let mut edges = vec![];
    let mut computers = HashSet::new();

    input.trim().lines().map(|l| l.trim()).for_each(|l| {
        let (left, right) = l.split_once("-").unwrap();
        let c = vec![left.to_string(), right.to_string()];
        let e = (left.to_string(), right.to_string());

        edges.push(e);
        computers.extend(c);
    });

    let computers = computers;
    let edges: Vec<(&str, &str, i32)> = edges
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str(), 1))
        .collect();

    let graph = UnGraphMap::from_edges(edges);

    // print dot file
    // println!("{:?}", petgraph::dot::Dot::new(&graph));

    let mut groups = HashSet::new();

    for node in graph.nodes().filter(|n| n.starts_with('t')) {
        let paths = search(&graph, node, 3);

        for path in paths.iter().filter(|p| *p.last().unwrap() == node) {

            let group: (&str, &str, &str) = path
                .iter()
                .cloned()
                .unique()
                .sorted()
                .collect_tuple()
                .unwrap();
            groups.insert(group);
        }
    }

    let part_1 = groups
        .iter()
        .filter(|(a, b, c)| *&[a, b, c].iter().any(|n| n.contains("t")))
        .count();

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
