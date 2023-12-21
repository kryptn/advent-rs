use advent::input_store;
use advent_toolbox::{
    automata::Automata,
    spatial::{Coordinate, Space, Traversable},
};

const YEAR: usize = 2023;
const DAY: usize = 21;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Square {
    Active,
    #[default]
    Plot,
    Rock,
}

impl Square {
    fn next_state(&self, neighbors: Vec<&Square>) -> Self {
        match self {
            Self::Rock => Self::Rock,
            Self::Plot | Self::Active => {
                if neighbors.iter().filter(|s| ***s == Self::Active).count() >= 1 {
                    Square::Active
                } else {
                    Square::Plot
                }
            }
        }
    }
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Square::Plot,
            '#' => Square::Rock,
            'S' => Square::Active,
            'O' => Square::Active,
            _ => panic!("Invalid square"),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Plot => write!(f, "."),
            Square::Rock => write!(f, "#"),
            Square::Active => write!(f, "O"),
        }
    }
}

fn apply(garden: &mut Space<Coordinate, Square>) {
    let mut changes = vec![];

    for (coord, square) in garden.iter() {
        let neighbors = coord
            .cardinals()
            .iter()
            .filter_map(|c| garden.get(c))
            .collect();
        let next_state = square.next_state(neighbors);
        if next_state != *square {
            changes.push((coord.clone(), next_state));
        }
    }

    for (coord, square) in changes {
        garden.insert(coord, square);
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"...........
    // .....###.#.
    // .###.##..#.
    // ..#.#...#..
    // ....#.#....
    // .##..S####.
    // .##..#...#.
    // .......##..
    // .##.#.####.
    // .##..##.##.
    // ..........."#;

    let mut garden: Space<Coordinate, Square> = Space::from(input);

    // println!("{}\n\n", garden);
    for _ in 0..64 {
        apply(&mut garden);
        // println!("{}\n\n", garden);
    }

    let part_1 = garden.values().filter(|s| **s == Square::Active).count();

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
