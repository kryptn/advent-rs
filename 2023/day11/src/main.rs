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

fn expand_row(space: &Space<Coordinate, Sector>) -> Space<Coordinate, Sector> {
    let mut out = vec![];
    let mut offset = 0;

    for row in space.rows() {
        let row: Vec<(Coordinate, Sector)> = row.map(|(c, &v)| (c, v)).collect();

        out.extend(
            row.iter()
                .cloned()
                .map(|(c, v)| (c + Coordinate::new(0, offset), v)),
        );

        if row.iter().all(|x| x.1 == Sector::Empty) {
            offset += 1;
            out.extend(
                row.iter()
                    .cloned()
                    .map(|(c, v)| (c + Coordinate::new(0, offset), v)),
            );
        }
    }

    out.into_iter().collect()
}

fn expand_columns(space: &Space<Coordinate, Sector>) -> Space<Coordinate, Sector> {
    let mut out = vec![];
    let mut offset = 0;

    for column in space.columns() {
        let column: Vec<(Coordinate, Sector)> = column.map(|(c, &v)| (c, v)).collect();

        out.extend(
            column
                .iter()
                .cloned()
                .map(|(c, v)| (c + Coordinate::new(offset, 0), v)),
        );

        if column.iter().all(|x| x.1 == Sector::Empty) {
            offset += 1;
            out.extend(
                column
                    .iter()
                    .cloned()
                    .map(|(c, v)| (c + Coordinate::new(offset, 0), v)),
            );
        }
    }

    out.into_iter().collect()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#....."#
    //         .to_string();

    let input = input.as_str();
    let space: Space<Coordinate, Sector> = Space::from(input);

    // println!("before expansion:\n{}", space);
    let space = expand_row(&space);
    let space = expand_columns(&space);

    // println!("after expansion:\n{}", space);

    let part_1: usize = space
        .iter()
        .filter_map(|(c, &v)| if v == Sector::Galaxy { Some(c) } else { None })
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.distance(b))
        .sum();

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
