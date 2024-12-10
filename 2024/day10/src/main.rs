use advent::input_store;
use advent_toolbox::{algo::dijkstra_all_paths, spatial::{Coordinate, Space}};

const YEAR: usize = 2024;
const DAY: usize = 10;



fn do_thing(outdoors: &Space<Coordinate, u32>) -> usize {
    let outdoors = outdoors.clone();

    let edges = |c: &Coordinate| c.cardinals().iter().filter(|n| {
        match (outdoors.get(c), outdoors.get(n)) {
            (Some(h), Some(nh)) => nh > h && nh-h == 1,
            _ => false,
        }
    }).map(|&a| a).collect();

    let is_goal = |c: &Coordinate| -> bool {
        match outdoors.get(c) {
            Some(h) => *h == 9,
            None => false,
        }
    };

    let cost_fn = |c: &Coordinate| -> Option<usize> {
        match outdoors.get(c) {
            Some(h) => Some(*h as usize),
            None => None,
        }
    };

    let trailheads = outdoors.iter().filter(|(c, h)| **h == 0).map(|(c, _)| *c).collect::<Vec<Coordinate>>();
    // dbg!(&trailheads);
    let mut paths = vec![];

    for trailhead in trailheads {
        let path = dijkstra_all_paths(&[trailhead], edges, is_goal, Some(cost_fn));
        paths.extend(path);
    }

    paths.len()
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
    let outdoors = outdoors.iter().map(|(c, h)| (*c, h.to_digit(10).unwrap())).collect();

    // println!("{}", outdoors);

    let part_1 = do_thing(&outdoors);

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
