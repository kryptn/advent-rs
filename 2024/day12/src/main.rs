use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};

const YEAR: usize = 2024;
const DAY: usize = 12;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
//     let input = r#"RRRRIICCFF
// RRRRIICCCF
// VVRRRCCFFF
// VVRCCCJFFF
// VVVVCJJCFE
// VVIVCCJJEE
// VVIIICJJEE
// MIIIIIJJEE
// MIIISIJEEE
// MMMISSJEEE
// "#;

    let fields: Space<Coordinate, char> = input.into();
    let mut visited: Space<Coordinate, bool> = Space::new();


    let mut part_1 = 0;

    for (coord, plot) in fields.iter() {
        if visited.contains_key(&coord) {
            continue;
        }

        let edges = |space: &Space<Coordinate, char>, node: &Coordinate| -> Vec<Coordinate> {
            node.cardinals()
                .iter()
                .filter_map(|n| match space.get(n) {
                    Some(p) => {
                        if plot == p {
                            Some(n.clone())
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect()
        };
        let nodes = fields.bfs(coord, &edges);
        for node in nodes.iter() {
            visited.insert(node.clone(), true);
        }
        let area = nodes.len() as u64;
        let perimeter: u64 = nodes
            .iter()
            .map(|n| n.cardinals().iter().filter(|c| !nodes.contains(c)).count() as u64)
            .sum();
        part_1 += area * perimeter;
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
