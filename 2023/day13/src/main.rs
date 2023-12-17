use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Square {
    #[default]
    Ash,
    Rock,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Square::Ash,
            '#' => Square::Rock,
            _ => panic!("invalid square: {}", c),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Square::Ash => '.',
            Square::Rock => '#',
        };
        write!(f, "{}", c)
    }
}

struct ScanResult {
    line: Coordinate,
    matching: isize,
    cells: isize,
    broken: bool,
    tolerated: isize,
}

impl ScanResult {
    fn score(&self) -> isize {
        if self.line.x > 0 {
            self.line.x
        } else {
            (self.line.y) * 100
        }
    }
}

impl std::fmt::Display for ScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line: ({}, {}), matching: {}, cells: {}, broken: {}, score: {}, tolerated: {}",
            self.line.x,
            self.line.y,
            self.matching,
            self.cells,
            self.broken,
            self.score(),
            self.tolerated,
        )
    }
}

fn scan_rows_match(
    grid: &Space<Coordinate, Square>,
    initial: isize,
    tolerations: isize,
) -> ScanResult {
    let (lower_bound, upper_bound) = grid.bounding_box();
    let mut matching_rows = 0;
    let mut matching_cells = 0;
    let mut broken = false;
    let mut tolerated = 0;

    'matching: while initial - matching_rows >= 0 {
        for x in lower_bound.x..=upper_bound.x {
            let a = grid.get(&(x, initial - matching_rows).into());
            let b = grid.get(&(x, initial + matching_rows + 1).into());
            match (a, b) {
                (Some(sa), Some(sb)) => {
                    if sa == sb {
                        matching_cells += 1;
                    } else {
                        if tolerated >= tolerations {
                            broken = true;
                            break 'matching;
                        } else {
                            tolerated += 1;
                        }
                    }
                }
                _ => break 'matching,
            }
        }
        matching_rows += 1;
    }

    ScanResult {
        line: (0, initial + 1).into(),
        matching: matching_rows,
        cells: matching_cells,
        broken,
        tolerated,
    }
}

fn scan_columns_match(
    grid: &Space<Coordinate, Square>,
    initial: isize,
    tolerations: isize,
) -> ScanResult {
    let (lower_bound, upper_bound) = grid.bounding_box();
    let mut matching_columns = 0;
    let mut matching_cells = 0;
    let mut broken = false;
    let mut tolerated = 0;

    'matching: while initial - matching_columns >= 0 {
        for y in lower_bound.y..=upper_bound.y {
            let a = grid.get(&(initial - matching_columns, y).into());
            let b = grid.get(&(initial + matching_columns + 1, y).into());
            match (a, b) {
                (Some(sa), Some(sb)) => {
                    if sa == sb {
                        matching_cells += 1;
                    } else {
                        if tolerated >= tolerations {
                            broken = true;
                            break 'matching;
                        } else {
                            tolerated += 1;
                        }
                    }
                }
                _ => break 'matching,
            }
        }
        matching_columns += 1;
    }

    ScanResult {
        line: (initial + 1, 0).into(),
        matching: matching_columns,
        cells: matching_cells,
        broken,
        tolerated,
    }
}

fn score_for_grid(grid: &Space<Coordinate, Square>, tolerations: isize) -> isize {
    let (lower, upper) = grid.bounding_box();

    let matching_rows = (lower.y..=upper.y).map(|y| scan_rows_match(&grid, y, tolerations));
    let matching_columns = (lower.x..=upper.x).map(|x| scan_columns_match(&grid, x, tolerations));

    let scores = matching_rows
        .chain(matching_columns)
        .sorted_by(|a, b| b.cells.cmp(&a.cells))
        .collect::<Vec<_>>();

    // println!("grid: \n{}", grid);

    for sr in &scores {
        // println!("{}", sr);
    }

    let valid: Vec<_> = scores
        .iter()
        .filter(|sr| !sr.broken && sr.matching > 0 && sr.tolerated == tolerations)
        .collect();

    // println!("valid: \n");
    for sr in &valid {
        // println!("{}", sr);
    }

    match valid.first() {
        Some(top) => {
            // println!("\ntop: {}\n\n", top);

            top.score()
        }
        None => {
            // println!("\nno top\n\n");

            0
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let grids: Vec<Space<Coordinate, Square>> = input
        .trim()
        .split("\n\n")
        .map(|rg| Space::from(rg))
        .collect();

    let part_1 = grids.iter().map(|g| score_for_grid(g, 0)).sum::<isize>();

    println!("part_1 => {}", part_1);

    let part_2 = grids.iter().map(|g| score_for_grid(g, 1)).sum::<isize>();
    println!("part_2 => {}", part_2);
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
