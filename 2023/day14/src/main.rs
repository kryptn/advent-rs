use std::collections::HashSet;

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
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

const UP: Coordinate = Coordinate { x: 0, y: -1 };
const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
const LEFT: Coordinate = Coordinate { x: -1, y: 0 };
const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };

fn rows(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.y..=upper.y)
        .map(move |y| (lower.x..=upper.x).map(|x| (x, y).into()).collect_vec())
        .flatten()
        .collect()
}

fn rows_rev(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.y..=upper.y)
        .rev()
        .map(move |y| (lower.x..=upper.x).map(|x| (x, y).into()).collect_vec())
        .flatten()
        .collect()
}

fn columns(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.x..=upper.x)
        .map(move |x| (lower.y..=upper.y).map(|y| (x, y).into()).collect_vec())
        .flatten()
        .collect()
}

fn columns_rev(mirror: &Space<Coordinate, Mirror>) -> Vec<Coordinate> {
    let (lower, upper) = mirror.bounding_box();

    (lower.x..=upper.x)
        .rev()
        .map(move |x| (lower.y..=upper.y).map(|y| (x, y).into()).collect_vec())
        .flatten()
        .collect()
}

fn tilt(mirror: &mut Space<Coordinate, Mirror>, direction: Coordinate) -> bool {
    let coords = match direction {
        UP => rows(mirror),
        DOWN => rows_rev(mirror),
        LEFT => columns_rev(mirror),
        RIGHT => columns(mirror),
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
    let dirs = &[UP, LEFT, DOWN, RIGHT];

    for d in dirs {
        while tilt(mirror, *d) {}
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

    // println!("{input}");
    // println!("{mirror}");

    while tilt(&mut mirror, UP) {
        // println!("{mirror}\n\n");
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // println!("{mirror}\n\n");

    println!("part_1 => {}", load(&mirror));

    let mut mirror: Space<Coordinate, Mirror> = Space::from(&input);

    let mut seen: HashSet<Vec<Coordinate>> = HashSet::new();
    let mut idx = 0;
    let mut first_cycle: Option<Vec<Coordinate>> = None;
    let mut first_cycle_idx = 0;
    while idx < 1000000000 {
        cycle(&mut mirror);
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
                let cycles = (1000000000 - idx) / cycle_len;
                idx += cycles * cycle_len;
            }
        }

        seen.insert(round_rocks);
        idx += 1;
        // println!("{idx}: {}\n{mirror}\n\n", seen.len());
    }

    // for d in &[UP, LEFT, DOWN, RIGHT] {
    //     let mut mirror = Space::from(&input);
    //     while tilt(&mut mirror, *d) {}
    //     println!("{d}\n{mirror}\n\n");
    // }

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
