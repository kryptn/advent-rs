use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Point, Space};

const YEAR: usize = 2024;
const DAY: usize = 04;

const DIRECTIONS: [Coordinate; 8] = [
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 1, y: 1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: 0, y: -1 },
    Coordinate { x: -1, y: -1 },
    Coordinate { x: -1, y: 0 },
    Coordinate { x: -1, y: 1 },
];

const MARGIN: Coordinate = Coordinate { x: 3, y: 3 };

fn word_from(point: Coordinate, direction: Coordinate) -> [Coordinate; 4] {
    let mut result = [Coordinate::default(); 4];
    for i in 0..4 {
        result[i] = point + direction * i as isize;
    }
    result
}

fn word_from_p2(point: Coordinate, direction: Coordinate) -> [Coordinate; 5] {
    let a = point + direction + direction.flip();
    let b = point + direction - direction.flip();
    let c = point;
    let d = point - direction + direction.flip();
    let e = point - direction - direction.flip();

    [a, b, c, d, e]
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX
    // "#;

    let search: Space<Coordinate, char> = Space::from(input);
    let mut found = 0;

    let (a, b) = search.bounding_box();
    for point in a.range(&b) {
        for direction in DIRECTIONS {
            let word: String = word_from(point, direction)
                .iter()
                .filter_map(|c| {
                    // println!("lookin for {:?}", c);
                    search.get(&c)
                })
                .collect();

            // println!("found {} at {:?} going {:?}", word, point, direction);

            if word == "XMAS" {
                found += 1;
            }
        }
    }

    println!("part_1 => {}", found);

    let mut found = 0;
    for point in a.range(&b) {
        for direction in DIRECTIONS {
            let word: String = word_from_p2(point, direction)
                .iter()
                .filter_map(|c| search.get(&c))
                .collect();

            if word == "MMASS" {
                found += 1;
            }
        }
    }

    println!("part_2 => {}", found);
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
