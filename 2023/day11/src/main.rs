use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Sector {
    Galaxy,
    Empty,
}

impl Default for Sector {
    fn default() -> Self {
        Sector::Empty
    }
}

impl std::fmt::Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sector::Galaxy => write!(f, "#"),
            Sector::Empty => write!(f, "."),
        }
    }
}

impl From<char> for Sector {
    fn from(c: char) -> Self {
        match c {
            '#' => Sector::Galaxy,
            '.' => Sector::Empty,
            _ => panic!("invalid sector"),
        }
    }
}

fn expand_row(space: &Space<Coordinate, Sector>, expand_by: isize) -> Space<Coordinate, Sector> {
    let mut out = vec![];
    let mut offset = 0;

    for row in space.rows() {
        let row: Vec<Coordinate> = row
            .filter_map(|(c, &v)| {
                if v == Sector::Galaxy {
                    Some(c + Coordinate::new(0, offset))
                } else {
                    None
                }
            })
            .collect();

        if row.is_empty() {
            offset += expand_by;
            continue;
        }

        out.extend(row);
    }

    out.into_iter().map(|c| (c, Sector::Galaxy)).collect()
}

fn expand_columns(
    space: &Space<Coordinate, Sector>,
    expand_by: isize,
) -> Space<Coordinate, Sector> {
    let mut out = vec![];
    let mut offset = 0;

    for column in space.columns() {
        let column: Vec<Coordinate> = column
            .filter_map(|(c, &v)| {
                if v == Sector::Galaxy {
                    Some(c + Coordinate::new(offset, 0))
                } else {
                    None
                }
            })
            .collect();

        if column.is_empty() {
            offset += expand_by;
            continue;
        }

        out.extend(column);
    }

    out.into_iter().map(|c| (c, Sector::Galaxy)).collect()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let inputctring();

    let input = input.as_str();
    let space: Space<Coordinate, Sector> = Space::from(input);

    // println!("before expansion:\n{}", space);
    let spacep1 = expand_row(&space, 1);
    let spacep1 = expand_columns(&spacep1, 1);

    // println!("after expansion:\n{}", space);

    let part_1: usize = spacep1
        .iter()
        .filter_map(|(c, &v)| if v == Sector::Galaxy { Some(c) } else { None })
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.distance(b))
        .sum();

    println!("part_1 => {}", part_1);

    // println!("before expansion:\n{}", space);
    let spacep2 = expand_row(&space, 1000000 - 1);
    let spacep2 = expand_columns(&spacep2, 1000000 - 1);

    // println!("after expansion:\n{}", space);

    let part_2: usize = spacep2
        .iter()
        .filter_map(|(c, &v)| if v == Sector::Galaxy { Some(c) } else { None })
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.distance(b))
        .sum();
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
