use std::collections::HashMap;

use advent::input_store;
use itertools::Itertools;
use ndarray::Array3;

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: u16,
    edges: Vec<&'a str>,
}

fn to_valve(input: &str) -> Valve {
    let (name, flow, _, edges) = sscanf::sscanf!(
        input,
        "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}",
    )
    .unwrap();
    let edges = edges.split(", ").collect();
    Valve { name, flow, edges }
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
        .map(|line| to_valve(line.trim()))
        .sorted_by(|a, b| b.flow.cmp(&a.flow))
        .collect();

    let name_to_idx: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(idx, v)| (v.name, idx))
        .collect();
    let edges: Vec<Vec<usize>> = valves
        .iter()
        .map(|v| v.edges.iter().map(|e| name_to_idx[e]).collect())
        .collect();

    let valves_with_flow = valves.iter().filter(|v| v.flow > 0).count();

    let aa_idx = name_to_idx["AA"];

    // in binary this would be valves_with_flow-many 1's
    let all_on = (1 << valves_with_flow) - 1;

    let mut space = Array3::<u16>::zeros([30, valves.len(), all_on + 1]);

    // bottom up, t is time remaining
    for t in 1..30 {
        // for every valve
        for v_idx in 0..valves.len() {
            let v_idx_mask = 1 << v_idx;

            // for any possible valve state
            for v_state in 0..=all_on {
                let mut flow = space[(t, v_idx, v_state)];

                // if valve is available and there's at least time to turn on and collect
                // then update the flow total
                if v_state & v_idx_mask != 0 && t >= 2 {
                    flow = flow.max(
                        space[(t - 1, v_idx, v_state - v_idx_mask)] + valves[v_idx].flow * t as u16,
                    );
                }

                // if any step from here would give us more flow, use it
                for &e in edges[v_idx].iter() {
                    flow = flow.max(space[(t - 1, e, v_state)])
                }
                space[(t, v_idx, v_state)] = flow;
            }
        }
    }

    let part_1 = space[(29, aa_idx, all_on)];
    println!("part_1 => {}", part_1);

    let mut part_2 = 0;
    for a in 0..((all_on + 1) / 2) {
        // take away any valves 'a' has selected
        let b = all_on - a;
        part_2 = part_2.max(space[(25, aa_idx, a)] + space[(25, aa_idx, b)])
    }

    println!("part_2 => {}", part_2)
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
