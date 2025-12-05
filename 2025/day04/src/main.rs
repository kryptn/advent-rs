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

    let grid: Space<Coordinate, Item> = Space::from_lines(&input);

    let part_1 = grid
        .iter()
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
        .count();

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
