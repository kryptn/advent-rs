use advent::input_store;
use advent_toolbox::{
    algo::dijkstra,
    spatial::{coordinates_within, Coordinate, Space},
};

const YEAR: usize = 2024;
const DAY: usize = 18;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
enum Tile {
    #[default]
    Corrupted,
    Safe,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Corrupted => write!(f, "#"),
            Tile::Safe => write!(f, "."),
        }
    }
}

fn path_for(
    grid: &Space<Coordinate, Tile>,
    start: Coordinate,
    end: Coordinate,
) -> Option<Vec<Coordinate>> {
    // gross. my dijkstra unwraps and dies when it shouldn't.
    let edges = |pos: &Coordinate| -> Vec<Coordinate> {
        pos.cardinals()
            .iter()
            .filter(|c| matches!(grid.get(c), Some(Tile::Safe)))
            .map(|c| *c)
            .collect::<Vec<_>>()
    };

    let is_goal = |c: &Coordinate| -> bool { *c == end };
    let cost = |_: &Coordinate| -> Option<usize> { Some(1) };

    let result = dijkstra(&[start], edges, is_goal, Some(cost));
    let node = result
        .costs
        .iter()
        .filter(|(k, _)| is_goal(*k))
        .min_by_key(|v| v.1)
        .map(|(k, _)| *k)?;

    let mut path =
        std::iter::successors(Some(node), |n| result.prev_map.get(n).copied()).collect::<Vec<_>>();
    path.reverse();
    Some(path)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let falling_bytes: Vec<Coordinate> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<Coordinate>().unwrap())
        .collect();

    let first_kb = falling_bytes.iter().take(1024).collect::<Vec<_>>();
    let start = Coordinate::new(0, 0);
    let end = Coordinate::new(70, 70);

    let mut grid: Space<Coordinate, Tile> = coordinates_within(start, end)
        .iter()
        .map(|c| {
            (
                *c,
                if first_kb.contains(&c) {
                    Tile::Corrupted
                } else {
                    Tile::Safe
                },
            )
        })
        .collect();

    let path = path_for(&grid, start, end).unwrap();
    println!("part_1 => {}", path.len() - 1);

    let mut part_2 = None;
    for byte in falling_bytes {
        grid.insert(byte, Tile::Corrupted);
        match path_for(&grid, start, end) {
            Some(_) => {}
            None => {
                part_2 = Some(byte);
                break;
            }
        }
    }

    let part_2 = part_2.unwrap();

    println!("part_2 => {},{}", part_2.x, part_2.y);
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
