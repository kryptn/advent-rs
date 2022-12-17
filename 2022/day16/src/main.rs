use std::collections::HashMap;

use advent::input_store;
use petgraph::{
    dot::{Config, Dot},
    prelude::UnGraph,
    stable_graph::{node_index, NodeIndex},
};

#[derive(Clone)]
struct Valve {
    name: String,
    rate: usize,
    leads_to: Vec<String>,
}

fn to_node_index(input: &str) -> NodeIndex {
    let v = input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let ch = c as usize - 65;
            let offset = (1000 as usize).pow(i as u32);
            let val = c as usize * offset;
            println!("{ch} * {offset} == {val}");
            val
        })
        .sum();

    println!("{input} -> {v}");

    node_index(v)
}

impl From<&str> for Valve {
    fn from(input: &str) -> Self {
        let pruned: String = input
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
struct Valves(HashMap<String, Valve>);

impl From<Vec<Valve>> for Valves {
    fn from(valves: Vec<Valve>) -> Self {
        Self(valves.into_iter().map(|v| (v.name.clone(), v)).collect())
    }
}

impl Valves {
    fn all_edges(&self) -> Vec<(String, String)> {
        self.0.values().map(|v| v.edges()).flatten().collect()
    }

    fn as_graph(self) -> UnGraph<Valve, usize> {
        let mut graph = UnGraph::default();

        let mut name_idx = HashMap::new();

        for valve in self.0.values() {
            let idx = graph.add_node(valve.clone());
            name_idx.insert(valve.name.clone(), idx);
        }

        self.all_edges()
            .iter()
            .map(|(a, b)| {
                let a_idx = name_idx[a];
                let b_idx = name_idx[b];

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

        graph
    }
}

#[derive(Debug, Clone)]
enum Action {
    Move(String),
    Open(String),
    Wait(String),
}

impl Action {
    fn inner(&self) -> String {
        match self {
            Action::Move(v) => v.clone(),
            Action::Open(v) => v.clone(),
            Action::Wait(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Traversal<'a> {
    valve_map: &'a Valves,
    open_valves: Vec<String>,
    actions: Vec<Action>,
}

impl<'a> From<&'a Valves> for Traversal<'a> {
    fn from(valve_map: &'a Valves) -> Self {
        let open_valves = Vec::new();
        let actions = Vec::new();
        Self {
            valve_map,
            open_valves,
            actions,
        }
    }
}

impl<'a> Traversal<'a> {
    fn current_location(&self) -> String {
        if self.actions.is_empty() {
            return "AA".to_string();
        }

        self.actions.last().unwrap().inner()
    }

    fn done(&self) -> bool {
        if self.actions.is_empty() {
            return false;
        }
        match self.actions.last().unwrap() {
            Action::Wait(_) => true,
            _ => false,
        }
    }

    fn previously_in(&self, valve: &String) -> bool {
        if self.actions.len() < 2 {
            false
        } else {
            self.actions[self.actions.len() - 2].inner() == *valve
        }
    }

    fn connected_tunnels(&self) -> Vec<String> {
        let current_location = self.current_location();
        self.valve_map.0[&current_location].leads_to.clone()
    }

    fn all_open(&self) -> bool {
        let valid_valves = self.valve_map.0.values().filter(|v| v.rate > 0).count();
        self.open_valves.len() == valid_valves
    }

    fn next_actions(&self) -> Vec<Action> {
        let mut out = Vec::new();
        let current_location = self.current_location();

        if self.all_open() {
            out.push(Action::Wait(current_location));
            return out;
        }

        if !self.open_valves.contains(&current_location)
            && self.valve_map.0[&current_location].rate > 0
        {
            out.push(Action::Open(current_location));
        }

        out.extend(
            self.connected_tunnels()
                .iter()
                .filter(|v| !self.previously_in(v))
                .map(|t| Action::Move(t.clone())),
        );
        out
    }

    fn with_action(&self, action: Action) -> Self {
        let mut next = self.clone();

        if let Action::Open(valve) = &action {
            next.open_valves.push(valve.clone());
        }

        next.actions.push(action);

        // dbg!(&next);

        next
    }

    fn dfs(&self, depth: usize) -> Option<Vec<Self>> {
        if self.actions.len() > depth || self.done() {
            return None;
        }

        let mut out = Vec::new();

        for action in self.next_actions() {
            if let Action::Wait(_) = action {
                out.push(self.clone());
                continue;
            }

            // dbg!(self.actions.len(), &action);
            let down = self.with_action(action);
            let downs = down.dfs(depth);
            match downs {
                Some(d) => out.extend(d),
                _ => out.push(self.clone()),
            }
        }
        // dbg!(&out);
        Some(out)
    }
}

fn main() {
    let input = input_store::get_input(2022, 16);

    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II"#;

    let valves: Vec<Valve> = input.trim().lines().map(|line| line.into()).collect();
    let valve_map: Valves = valves.into();

    dbg!(&valve_map);

    let graph = valve_map.as_graph();

    let nf = |g, (idx, v)| format!("{:?}", v);
    let ef = |g, e| "".to_string();

    let v = Dot::with_attr_getters(&graph, &[], &ef, &nf);
    let v = Dot::with_config(&graph, &[]);

    println!("{:?}", v);

    dbg!(graph.node_count());

    // let root: Traversal = (&valve_map).into();
    // dbg!(root.next_actions());

    // let traversals = root.dfs(30).unwrap();
    // dbg!(traversals.len());

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
