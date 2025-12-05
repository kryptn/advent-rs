use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Direction, Space};

const YEAR: usize = 2025;
const DAY: usize = 04;

enum Item {
    Empty,
    Occupied,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            '@' => Item::Occupied,
            '.' => Item::Empty,
            _ => panic!("unknown item char {}", c),
        }
    }
}

fn removal_candidates(grid: &Space<Coordinate, Item>) -> Vec<Coordinate> {
    grid.iter()
        .filter_map(|(key, value)| {
            if matches!(value, Item::Occupied) {
                Some(key)
            } else {
                None
            }
        })
        .filter(|coord| {
            let neighbors = coord
                .neighbors()
                .iter()
                .filter(|c| {
                    grid.get(c)
                        .and_then(|item| match item {
                            Item::Occupied => Some(()),
                            Item::Empty => None,
                        })
                        .is_some()
                })
                .count();

            neighbors < 4
        })
        .cloned()
        .collect()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.
    // "#;

    let mut grid: Space<Coordinate, Item> = Space::from_lines(&input);

    let mut removed = vec![];
    let mut candidates = removal_candidates(&grid);
    println!("part_1 => {}", candidates.len());

    while !candidates.is_empty() {
        for candidate in candidates.iter() {
            grid.insert(*candidate, Item::Empty);
            removed.push(*candidate);
        }

        candidates = removal_candidates(&grid);
    }

    println!("part_2 => {}", removed.len());
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
