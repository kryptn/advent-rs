use std::{
    collections::{HashMap, HashSet, VecDeque},
    marker::PhantomData,
    rc::Rc,
    sync::Arc,
};

use advent::input_store;
use itertools::Itertools;
use petgraph::{
    dot::{Config, Dot},
    graph::Externals,
    graphmap::{DiGraphMap, UnGraphMap},
    EdgeDirection::Outgoing,
};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Flavor {
    Amber,
    Bronze,
    Copper,
    Desert,
    Hallway,
}

impl Flavor {
    fn cost(&self) -> usize {
        match self {
            Flavor::Amber => 1,
            Flavor::Bronze => 10,
            Flavor::Copper => 100,
            Flavor::Desert => 1000,
            _ => unreachable!(),
        }
    }
}

impl Default for Flavor {
    fn default() -> Self {
        Self::Hallway
    }
}

impl From<char> for Flavor {
    fn from(c: char) -> Self {
        match c {
            'A' => Flavor::Amber,
            'B' => Flavor::Bronze,
            'C' => Flavor::Copper,
            'D' => Flavor::Desert,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Flavor::Amber => "A",
            Flavor::Bronze => "B",
            Flavor::Copper => "C",
            Flavor::Desert => "D",
            Flavor::Hallway => ".",
        };

        write!(f, "{}", v)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Room(Flavor, usize);

impl Room {
    fn new(flavor: Flavor, id: usize) -> Self {
        Self(flavor, id)
    }

    fn all() -> [Self; 15] {
        let h1 = Room::new(Flavor::Hallway, 1);
        let h2 = Room::new(Flavor::Hallway, 2);
        let a1 = Room::new(Flavor::Amber, 1);
        let a2 = Room::new(Flavor::Amber, 2);
        let h4 = Room::new(Flavor::Hallway, 4);
        let b1 = Room::new(Flavor::Bronze, 1);
        let b2 = Room::new(Flavor::Bronze, 2);
        let h6 = Room::new(Flavor::Hallway, 6);
        let c1 = Room::new(Flavor::Copper, 1);
        let c2 = Room::new(Flavor::Copper, 2);
        let h8 = Room::new(Flavor::Hallway, 8);
        let d1 = Room::new(Flavor::Desert, 1);
        let d2 = Room::new(Flavor::Desert, 2);
        let h10 = Room::new(Flavor::Hallway, 10);
        let h11 = Room::new(Flavor::Hallway, 11);

        [h1, h2, a1, a2, h4, b1, b2, h6, c1, c2, h8, d1, d2, h10, h11]
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State([(Room, Flavor); 8]);

impl State {
    fn occupied(&self, room: Room) -> bool {
        self.0.iter().any(|(r, _)| *r == room)
    }

    fn valid_steps(&self, graph: &DiGraphMap<Room, (usize, Policy)>) -> Vec<(Self, Self, usize)> {
        let mut out = Vec::new();
        for (i, (room, agent)) in self.0.iter().enumerate() {
            for (a, edge, (weight, policy)) in graph.edges_directed(*room, Outgoing) {
                if policy.permits(*agent) && !self.occupied(edge) {
                    let mut next_state = self.clone();
                    next_state.0[i] = (edge, *agent);

                    let cost = agent.cost() * weight;

                    out.push((*self, next_state, cost));
                }
            }
        }

        out
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0.map(|v| v.1).iter().join(",");
        f.debug_tuple("State").field(&value).finish()
    }
}

// impl std::fmt::Display for State {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let map: HashMap<Room, Flavor> = self.0.iter().cloned().collect();

//         let all_rooms = Room::all();
//         let mut rooms = all_rooms
//             .iter()
//             .map(|r| map.get(r).unwrap_or(&Flavor::Hallway));
//         let (h1, h2, a1, a2, h4, b1, b2, h6) = rooms.next_tuple().unwrap();
//         let (c1, c2, h8, d1, d2, h10, h11) = rooms.next_tuple().unwrap();

//         write!(
//             f,
//             r#"#############
// #{}{}.{}.{}.{}.{}{}#
// ###{}#{}#{}#{}###
//   #{}#{}#{}#{}#
//   #########"#,
//             h1, h2, h4, h6, h8, h10, h11, a1, b1, c1, d1, a2, b2, c2, d2
//         )

//     }
// }

#[derive(Debug, Clone)]
enum Policy {
    None,
    Only(Flavor),
    Not(Flavor),
}

impl Policy {
    fn permits(&self, flavor: Flavor) -> bool {
        match self {
            Policy::None => true,
            Policy::Only(f) => *f == flavor,
            Policy::Not(f) => *f != flavor,
        }
    }
}

impl From<String> for State {
    fn from(input: String) -> Self {
        let a1 = Room::new(Flavor::Amber, 1);
        let a2 = Room::new(Flavor::Amber, 2);
        let b1 = Room::new(Flavor::Bronze, 1);
        let b2 = Room::new(Flavor::Bronze, 2);
        let c1 = Room::new(Flavor::Copper, 1);
        let c2 = Room::new(Flavor::Copper, 2);
        let d1 = Room::new(Flavor::Desert, 1);
        let d2 = Room::new(Flavor::Desert, 2);

        let rooms = [a1, b1, c1, d1, a2, b2, c2, d2];
        let agents = input
            .chars()
            .filter(|c| "ABCD".chars().contains(c))
            .map(|c| Flavor::from(c));
        let (a, b, c, d, e, f, g, h) = rooms.iter().cloned().zip(agents).next_tuple().unwrap();

        Self([a, b, c, d, e, f, g, h])
    }
}

#[allow(unused_variables)]
fn build_graph() -> DiGraphMap<Room, (usize, Policy)> {
    let h1 = Room::new(Flavor::Hallway, 1);
    let h2 = Room::new(Flavor::Hallway, 2);
    let a1 = Room::new(Flavor::Amber, 1);
    let a2 = Room::new(Flavor::Amber, 2);
    let h4 = Room::new(Flavor::Hallway, 4);
    let b1 = Room::new(Flavor::Bronze, 1);
    let b2 = Room::new(Flavor::Bronze, 2);
    let h6 = Room::new(Flavor::Hallway, 6);
    let c1 = Room::new(Flavor::Copper, 1);
    let c2 = Room::new(Flavor::Copper, 2);
    let h8 = Room::new(Flavor::Hallway, 8);
    let d1 = Room::new(Flavor::Desert, 1);
    let d2 = Room::new(Flavor::Desert, 2);
    let h10 = Room::new(Flavor::Hallway, 10);
    let h11 = Room::new(Flavor::Hallway, 11);

    let edges = [
        // h1
        (h1, h2, (1, Policy::None)),
        // h2
        (h2, h1, (1, Policy::None)),
        (h2, h4, (2, Policy::None)),
        (h2, a1, (2, Policy::Only(Flavor::Amber))),
        // a1
        (a1, h2, (2, Policy::None)),
        (a1, h4, (2, Policy::None)),
        (a1, a2, (1, Policy::Only(Flavor::Amber))),
        // a2
        (a2, a1, (1, Policy::Not(Flavor::Amber))),
        // h4
        (h4, h2, (2, Policy::None)),
        (h4, h6, (2, Policy::None)),
        (h4, a1, (2, Policy::Only(Flavor::Amber))),
        (h4, b1, (2, Policy::Only(Flavor::Bronze))),
        // b1
        (b1, h4, (2, Policy::None)),
        (b1, h6, (2, Policy::None)),
        (b1, b2, (1, Policy::Only(Flavor::Bronze))),
        // b2
        (b2, b1, (1, Policy::Not(Flavor::Bronze))),
        // h6
        (h6, h4, (2, Policy::None)),
        (h6, h8, (2, Policy::None)),
        (h6, b1, (2, Policy::Only(Flavor::Bronze))),
        (h6, c1, (2, Policy::Only(Flavor::Copper))),
        // c1
        (c1, h6, (2, Policy::None)),
        (c1, h8, (2, Policy::None)),
        (c1, c2, (1, Policy::Only(Flavor::Copper))),
        // c2
        (c2, c1, (1, Policy::Not(Flavor::Copper))),
        // h8
        (h8, h6, (2, Policy::None)),
        (h8, h10, (2, Policy::None)),
        (h8, c1, (2, Policy::Only(Flavor::Copper))),
        (h8, d1, (2, Policy::Only(Flavor::Desert))),
        // d1
        (d1, h8, (2, Policy::None)),
        (d1, h10, (2, Policy::None)),
        (d1, d2, (1, Policy::Only(Flavor::Desert))),
        // d2
        (d2, d1, (1, Policy::Not(Flavor::Desert))),
        // h10
        (h10, h8, (2, Policy::None)),
        (h10, h11, (2, Policy::None)),
        (h10, d1, (2, Policy::Only(Flavor::Desert))),
        // h11
        (h11, h10, (1, Policy::None)),
    ];

    let graph = DiGraphMap::from_edges(edges);

    graph
}

// fn inner_build_state_graph(position: State, state_graph: &mut DiGraphMap<State, usize>, graph: &DiGraphMap<Room, (usize, Policy)>) {
//     let mut staged = initial.valid_steps(graph);

//     while staged.len() > 0 {
//         let (next, cost) = staged.pop().unwrap();
//         state_graph.add_edge(a, b, weight)

//     }

// }

fn build_state_graph(
    initial: State,
    target: State,
    graph: &DiGraphMap<Room, (usize, Policy)>,
) -> DiGraphMap<State, usize> {
    let mut state_graph: DiGraphMap<State, usize> = DiGraphMap::new();
    state_graph.add_node(initial);

    //let mut staged: VecDeque<_> = initial.valid_steps(graph).into();

    let mut path = vec![initial];

    loop {}

    loop {
        let mut saw = 0;

        for node in state_graph.nodes() {
            println!("{}, {}", state_graph.node_count(), state_graph.edge_count());
            if node == target {
                continue;
            }
            saw += 1;

            for (this, next, cost) in node.valid_steps(graph) {
                if state_graph.contains_node(next) {
                    continue;
                }
            }

            let next_valids = node
                .valid_steps(graph)
                .iter()
                .filter(|(_, n, _)| !state_graph.contains_node(*n))
                .collect();
            for (this, next, cost) in next_valids {}

            state_graph.add_edge(this, next, cost);
            staged.extend(
                next.valid_steps(graph)
                    .iter()
                    .filter(|(_, n, _)| !state_graph.contains_node(*n)),
            )
        }
    }

    while staged.len() > 0 {
        println!("{}, {}", staged.len(), state_graph.node_count());
        let (this, next, cost) = staged.pop_front().unwrap();
        if this == target {
            continue;
        }

        state_graph.add_edge(this, next, cost);
        staged.extend(
            next.valid_steps(graph)
                .iter()
                .filter(|(_, n, _)| !state_graph.contains_node(*n)),
        )
    }

    state_graph
}

const TARGET: &str = r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"#;

fn main() {
    let input = input_store::get_input(2021, 23);

    let target: State = TARGET.to_string().into();

    let graph = build_graph();

    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let state: State = input.into();
    //dbg!(&state);

    //println!("start:\n{}\n\n", state);

    let state_graph = build_state_graph(state, target, &graph);
    //println!("{:?}", Dot::with_config(&state_graph, &[Config::EdgeNoLabel]));

    println!(
        "state graph: {} nodes, {} edges",
        state_graph.node_count(),
        state_graph.edge_count()
    );

    // for (_, s, cost) in state.valid_steps(&graph) {
    //     println!("cost: {}\n{}\n\n", cost, s);
    // }

    //dbg!(state.valid_steps(&graph));

    // println!("part_1 => {}", "not done");
    // println!("part_2 => {}", "not done");
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
