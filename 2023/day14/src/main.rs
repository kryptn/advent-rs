use std::collections::HashSet;

use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Mirror {
    Round,
    Cube,
    #[default]
    Empty,
    Marker,
}

impl From<char> for Mirror {
    fn from(c: char) -> Self {
        match c {
            '.' => Mirror::Empty,
            'O' => Mirror::Round,
            '#' => Mirror::Cube,
            _ => panic!("invalid mirror: {}", c),
        }
    }
}

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Mirror::Empty => '.',
            Mirror::Round => 'O',
            Mirror::Cube => '#',
            Mirror::Marker => 'X',
        };
        write!(f, "{}", c)
    }
}

fn rows(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.y..=upper.y)
        .rev()
        .map(move |y| (lower.x..=upper.x).map(move |x| (x, y).into()))
        .flatten()
        .collect()
}

fn rows_rev(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.y..=upper.y)
        .map(move |y| (lower.x..=upper.x).map(move |x| (x, y).into()))
        .flatten()
        .collect()
}

fn columns(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.x..=upper.x)
        .map(move |x| (lower.y..=upper.y).map(move |y| (x, y).into()))
        .flatten()
        .collect()
}

fn columns_rev(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.x..=upper.x)
        .rev()
        .map(move |x| (lower.y..=upper.y).map(move |y| (x, y).into()))
        .flatten()
        .collect()
}

fn tilt(mirror: &mut Space<Coordinate, Mirror>, direction: Coordinate) -> bool {
    let coords = match direction {
        spatial::UP => rows(mirror),
        spatial::DOWN => rows_rev(mirror),
        spatial::LEFT => columns_rev(mirror),
        spatial::RIGHT => columns(mirror),
        _ => panic!("invalid direction: {}", direction),
    };

    let mut modified = false;

    for c in coords {
        match mirror.get(&c).unwrap_or(&Mirror::Empty) {
            Mirror::Round => {}
            _ => continue,
        }
        // we can now assume that this is a round rock

        let c_next = c + direction;
        match mirror.get(&c_next) {
            Some(Mirror::Empty) => {
                // we can move the rock
                mirror.insert(c_next, Mirror::Round);
                mirror.insert(c, Mirror::Empty);
                modified = true;
            }
            _ => {}
        }
    }

    modified
}

fn load(mirror: &Space<Coordinate, Mirror>) -> isize {
    let (_, upper) = mirror.bounding_box();

    mirror
        .iter()
        .filter(|(_, v)| **v == Mirror::Round)
        .map(|(c, _)| upper.y - c.y + 1)
        .sum()
}

fn cycle(mirror: &mut Space<Coordinate, Mirror>) {
    let dirs = &[spatial::DOWN, spatial::LEFT, spatial::UP, spatial::RIGHT];

    for d in dirs {
        while tilt(mirror, *d) {}
    }
}

fn run_cycles(mirror: &mut Space<Coordinate, Mirror>, cycles: usize) {
    let mut seen: HashSet<Vec<Coordinate>> = HashSet::new();

    let mut idx = 0;
    let mut first_cycle: Option<Vec<Coordinate>> = None;
    let mut first_cycle_idx = 0;
    while idx < cycles {
        cycle(mirror);
        let round_rocks = mirror
            .iter()
            .filter(|(_, v)| **v == Mirror::Round)
            .map(|(&c, _)| c)
            .sorted()
            .collect::<Vec<_>>();
        if first_cycle.is_none() && seen.contains(&round_rocks) {
            first_cycle = Some(round_rocks.clone());
            first_cycle_idx = idx;
        } else if let Some(first_cycle) = &first_cycle {
            if first_cycle == &round_rocks {
                let cycle_len = idx - first_cycle_idx;
                let cycles = (cycles - idx) / cycle_len;
                idx += cycles * cycle_len;
            }
        }

        seen.insert(round_rocks);
        idx += 1;
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#...."#
    //     .to_string();

    let mut mirror: Space<Coordinate, Mirror> = Space::from(&input);
    while tilt(&mut mirror, spatial::DOWN) {}
    println!("part_1 => {}", load(&mirror));

    let mut mirror: Space<Coordinate, Mirror> = Space::from(&input);
    run_cycles(&mut mirror, 1000000000);
    println!("part_2 => {}", load(&mirror));
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
