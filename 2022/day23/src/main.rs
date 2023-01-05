use std::collections::HashSet;

use advent::input_store;
use advent_toolbox::spatial::{Cardinal, Coordinate, Space};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell(bool);

impl From<char> for Cell {
    fn from(input: char) -> Self {
        match input {
            '#' => Cell(true),
            _ => Cell(false),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            true => '#',
            false => '.',
        };
        write!(f, "{c}")
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self(false)
    }
}

type GridType = Space<Coordinate, Cell>;
struct Grid {
    grid: GridType,
    direction_idx: usize,

    round: usize,
}

const DIRECTIONS: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::South,
    Cardinal::West,
    Cardinal::East,
];

fn dir_by_idx(idx: usize) -> Cardinal {
    DIRECTIONS[(idx) % DIRECTIONS.len()]
}

impl Grid {
    fn occupied(&self, at: Coordinate, direction: &Cardinal) -> bool {
        let any_true = |d: &Cardinal| -> bool {
            let c = at + *d;
            let r = self.grid.get(&c).unwrap_or(&Cell::default()).0;
            // println!("    {at} -> {c} => {r}");
            r
        };

        match direction {
            Cardinal::North => {
                // println!("  {direction:?}");
                [Cardinal::NorthWest, Cardinal::North, Cardinal::NorthEast]
                    .iter()
                    .any(any_true)
            }
            Cardinal::East => {
                // println!("  {direction:?}");
                [Cardinal::NorthEast, Cardinal::East, Cardinal::SouthEast]
                    .iter()
                    .any(any_true)
            }
            Cardinal::South => {
                // println!("  {direction:?}");
                [Cardinal::SouthEast, Cardinal::South, Cardinal::SouthWest]
                    .iter()
                    .any(any_true)
            }
            Cardinal::West => {
                // println!("  {direction:?}");
                [Cardinal::SouthWest, Cardinal::West, Cardinal::NorthWest]
                    .iter()
                    .any(any_true)
            }
            _ => panic!("unexpected"),
        }
    }

    fn propose(&self, at: &Coordinate) -> Coordinate {
        // println!("{at}");
        let valid_steps: Vec<_> = (0..4)
            .into_iter()
            .map(|idx| dir_by_idx(idx + self.direction_idx))
            .filter(|d| !self.occupied(*at, d))
            .collect();

        // println!("{:?}\n\n", valid_steps);

        if valid_steps.len() == 4 || valid_steps.len() == 0 {
            at.clone()
        } else {
            at.clone() + valid_steps.first().unwrap().clone()
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid = GridType::from_lines_rev(input);
        let grid = grid
            .clone()
            .into_iter()
            .filter(|(_, cell)| cell.0)
            .collect();

        Self {
            grid,
            direction_idx: 0,
            round: 0,
        }
    }
}

impl Iterator for Grid {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let proposed: Vec<(&Coordinate, Coordinate)> =
            self.grid.keys().map(|c| (c, self.propose(c))).collect();
        let proposals: Vec<&Coordinate> = proposed.iter().map(|(_, p)| p).collect();

        let this_set: HashSet<&Coordinate> = self.grid.keys().collect();
        let that_set: HashSet<&Coordinate> = proposed
            .iter()
            .map(|(orig, proposal)| {
                if proposals.iter().filter(|c| **c == proposal).count() == 1 {
                    proposal
                } else {
                    orig
                }
            })
            .collect();

        self.round += 1;
        if this_set == that_set {
            None
        } else {
            self.grid = that_set
                .into_iter()
                .map(|c| (c.clone(), Cell(true)))
                .collect();
            self.direction_idx = (self.direction_idx + 1) % DIRECTIONS.len();

            Some(())
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 23);
    //     let input = r#"....#..
    // ..###.#
    // #...#.#
    // .#...##
    // #.###..
    // ##.#.##
    // .#..#.."#.to_string();
    let mut grid = Grid::from(input.as_str());

    for _ in 0..10 {
        grid.next().unwrap();
        // println!("{}\n\n", grid.grid);
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let (lower, upper) = grid.grid.bounding_box();
    let area = (upper.x + 1 - lower.x).abs() * (upper.y + 1 - lower.y).abs();
    let p1 = area - grid.grid.len() as isize;
    println!("part_1 => {}", p1);

    while let Some(_) = grid.next() {
        // println!("{}\n\n", grid.grid);
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }
    println!("part_2 => {}", grid.round);
}

#[cfg(test)]
mod test {

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
