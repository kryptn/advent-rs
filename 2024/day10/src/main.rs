use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
use petgraph::prelude::DiGraphMap;

const YEAR: usize = 2024;
const DAY: usize = 10;

fn do_thing_with_graph(outdoors: &Space<Coordinate, u32>) -> (usize, usize) {
    let mut g: DiGraphMap<Coordinate, u32> = DiGraphMap::new();

    for (c, _) in outdoors.iter() {
        let edges = c
            .cardinals()
            .iter()
            .filter(|n| match (outdoors.get(c), outdoors.get(n)) {
                (Some(h), Some(nh)) => nh > h && nh - h == 1,
                _ => false,
            })
            .map(|&a| a)
            .collect::<Vec<Coordinate>>();
        g.add_node(*c);
        for n in edges {
            g.add_edge(*c, n, 1);
        }
    }

    let trailheads: Vec<Coordinate> = outdoors
        .iter()
        .filter(|(_, h)| **h == 0)
        .map(|(c, _)| *c)
        .collect();

    let mut part_1 = 0;
    let mut part_2 = 0;

    for trailhead in trailheads.iter() {
        let path = petgraph::algo::dijkstra(&g, *trailhead, None, |(_, _, w)| *w);

        let nines: Vec<Coordinate> = path
            .iter()
            .filter(|(_, h)| **h == 9)
            .map(|(c, _)| *c)
            .collect();
        part_1 += nines.len();

        for nine in nines {
            part_2 += petgraph::algo::all_simple_paths::<
                Vec<Coordinate>,
                &DiGraphMap<Coordinate, u32>,
            >(&g, *trailhead, nine, 0, None)
            .count();
        }
    }

    (part_1, part_2)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"89010123
    // 78121874
    // 87430965
    // 96549874
    // 45678903
    // 32019012
    // 01329801
    // 10456732
    // "#;

    let outdoors: Space<Coordinate, char> = input.into();
    let outdoors: Space<Coordinate, u32> = outdoors
        .iter()
        .map(|(c, h)| (*c, h.to_digit(10).unwrap()))
        .collect();

    let answers = do_thing_with_graph(&outdoors);

    println!("part_1 => {}", answers.0);
    println!("part_2 => {}", answers.1);
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
