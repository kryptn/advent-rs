use std::{collections::HashMap, hash::Hasher, iter::FromIterator, sync::Arc};

use advent::input_store;
use itertools::{sorted, Itertools};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::alpha1,
    multi::separated_list1,
    IResult,
};
use petgraph::{
    algo::kosaraju_scc,
    dot::{Config, Dot},
    graph::UnGraph,
    Graph,
};

use std::hash::Hash;

const YEAR: usize = 2023;
const DAY: usize = 25;

fn parse_connections(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, from) = take_until(":")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, to) = separated_list1(tag(" "), alpha1)(input)?;
    // let (input, _) = tag("\n")(input)?;

    Ok((input, (from, to)))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let input = r#"jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr"#;

    let connections = input
        .trim()
        .lines()
        .map(|line| parse_connections(line.trim()).unwrap().1)
        .collect::<Vec<_>>();

    let mut indexes = HashMap::new();
    let mut edge_indexes = HashMap::new();

    let mut graph = UnGraph::<&str, bool>::new_undirected();
    for (from, to) in connections {
        let from_idx = if let Some(idx) = indexes.get(from) {
            *idx
        } else {
            let idx = graph.add_node(from);
            indexes.insert(from, idx);
            idx
        };

        for t in to {
            let to_idx = if let Some(idx) = indexes.get(t) {
                *idx
            } else {
                let idx = graph.add_node(t);
                indexes.insert(t, idx);
                idx
            };
            if !graph.contains_edge(from_idx, to_idx) {
                let (edge_key, (a, b)) = if from <= t {
                    ((from, t), (from_idx, to_idx))
                } else {
                    ((t, from), (to_idx, from_idx))
                };

                let edge_idx = graph.add_edge(a, b, true);
                edge_indexes.insert(edge_key, edge_idx);
            }
        }
    }

    let mut part_1 = 0;

    let edge_keys = edge_indexes.keys().cloned().collect::<Vec<_>>();

    for keys in edge_keys.iter().combinations(3) {
        for (a, b) in keys.iter() {
            let idx = edge_indexes.get(&(*a, *b)).unwrap();
            graph.remove_edge(*idx);
        }

        let r = kosaraju_scc(&graph);
        if r.len() == 2 {
            part_1 = r[0].len() * r[1].len();
            break;
        }

        for (a, b) in keys.iter() {
            let a_idx = indexes.get(a).unwrap();
            let b_idx = indexes.get(b).unwrap();
            let edge_idx = graph.add_edge(*a_idx, *b_idx, true);
            edge_indexes.insert((*a, *b), edge_idx);
        }
    }

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
