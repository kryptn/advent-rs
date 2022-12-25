use std::collections::HashMap;

use advent::input_store;
use itertools::Itertools;
use petgraph::{algo::astar, prelude::UnGraph, stable_graph::NodeIndex};
use rayon::prelude::{ParallelBridge, ParallelIterator};

#[derive(Clone)]
struct Valve {
    name: String,
    rate: usize,
    leads_to: Vec<String>,
}

impl From<&str> for Valve {
    fn from(input: &str) -> Self {
        let pruned: String = input
            .trim()
            .chars()
            .filter(|c| match c {
                'A'..='Z' | '0'..='9' | ' ' => true,
                _ => false,
            })
            .collect();
        let parts: Vec<&str> = pruned.trim().split_whitespace().collect();

        let name = parts[1].to_string();
        let rate = parts[2].parse().unwrap();
        let leads_to = parts[3..].iter().map(|p| p.to_string()).collect();

        Self {
            name,
            rate,
            leads_to,
        }
    }
}

impl Valve {
    fn edges(&self) -> Vec<(String, String)> {
        self.leads_to
            .iter()
            .map(|d| (self.name.clone(), d.clone()))
            .collect()
    }
}

impl std::fmt::Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.rate)
    }
}

#[derive(Debug)]
struct Valves {
    valves: HashMap<String, Valve>,
    valves_by_rate: Vec<String>,
    graph: UnGraph<Valve, usize>,
    idx_map: HashMap<String, NodeIndex>,
}

impl From<Vec<Valve>> for Valves {
    fn from(valves: Vec<Valve>) -> Self {
        let valves: HashMap<String, Valve> =
            valves.into_iter().map(|v| (v.name.clone(), v)).collect();

        let valves_by_rate = valves
            .values()
            .filter(|v| v.rate > 0)
            .sorted_by(|a, b| b.rate.cmp(&a.rate))
            .map(|v| v.name.clone())
            .collect();

        Self {
            valves,
            valves_by_rate,
            graph: UnGraph::default(),
            idx_map: HashMap::new(),
        }
    }
}

impl Valves {
    fn all_edges(&self) -> Vec<(String, String)> {
        self.valves.values().map(|v| v.edges()).flatten().collect()
    }

    fn as_graph(&mut self) {
        let mut graph = UnGraph::default();

        for valve in self.valves.values() {
            let idx = graph.add_node(valve.clone());
            self.idx_map.insert(valve.name.clone(), idx);
        }

        self.all_edges()
            .iter()
            .map(|(a, b)| {
                let a_idx = self.idx_map[a];
                let b_idx = self.idx_map[b];

                if a <= b {
                    (a_idx, b_idx)
                } else {
                    (b_idx, a_idx)
                }
            })
            .for_each(|(a, b)| {
                if !graph.contains_edge(a, b) {
                    graph.add_edge(a, b, 1);
                }
            });

        self.graph = graph;
    }

    fn distance(&self, start: &String, end: &String) -> Option<usize> {
        let start_idx = self.idx_map[start];
        let end_idx = self.idx_map[end];

        let path = astar(&self.graph, start_idx, |n| n == end_idx, |_| 1, |_| 1);
        match path {
            Some((len, _)) => Some(len as usize),
            None => None,
        }
    }

    fn follow_path(&self, path: &[String]) -> Vec<(usize, usize)> {
        let mut out = Vec::new();

        for (a, b) in path.iter().tuple_windows() {
            let b_rate = self.valves[b].rate;
            let distance = self.distance(a, b).unwrap();
            out.push((b_rate, distance));
        }

        out
    }
}

fn flow_for(input: Vec<(usize, usize)>, time: usize) -> usize {
    let mut t = 0;
    let mut flow_idx = 0;
    let mut rate = 0;
    let mut flow = 0;
    let mut dist = 0;

    while t <= time {
        flow += rate;

        if flow_idx < input.len() {
            let (r, d) = input[flow_idx];
            if dist > d {
                flow_idx += 1;
                rate += r;
                dist = 0;
                // println!("adding {rate} to flow");
            }
        }
        t += 1;
        dist += 1;

        // println!("minute {t}, flow rate: {rate}, flow total: {flow}");
    }

    flow
}

fn main() {
    let input = input_store::get_input(2022, 16);

    // let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    // Valve BB has flow rate=13; tunnels lead to valves CC, AA
    // Valve CC has flow rate=2; tunnels lead to valves DD, BB
    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    // Valve EE has flow rate=3; tunnels lead to valves FF, DD
    // Valve FF has flow rate=0; tunnels lead to valves EE, GG
    // Valve GG has flow rate=0; tunnels lead to valves FF, HH
    // Valve HH has flow rate=22; tunnel leads to valve GG
    // Valve II has flow rate=0; tunnels lead to valves AA, JJ
    // Valve JJ has flow rate=21; tunnel leads to valve II"#;

    let valves: Vec<Valve> = input
        .trim()
        .lines()
        .map(|line| Valve::from(line))
        .sorted_by(|a, b| b.rate.cmp(&a.rate))
        .collect();
    let mut valve_map: Valves = valves.clone().into();

    // dbg!(&valve_map);
    valve_map.as_graph();
    // dbg!(&valve_map);

    let mut paths: Vec<(Vec<String>, usize)> = valve_map
        .valves_by_rate
        .clone()
        .into_iter()
        .permutations(valve_map.valves_by_rate.len() - 8)
        .par_bridge()
        .map(|p| {
            let mut path = vec!["AA".to_string()];
            path.extend(p);
            let flow_map = valve_map.follow_path(path.as_slice());
            let flow = flow_for(flow_map, 30);
            (path, flow)
        })
        // .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect();

    paths.sort_by(|a, b| b.1.cmp(&a.1));

    let p1 = paths.first().unwrap();

    println!("part_1 => {}", p1.1);
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
