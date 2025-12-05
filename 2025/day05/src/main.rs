use advent::input_store;
use advent_toolbox::{parser_helpers::just_numbers, range::Range};

const YEAR: usize = 2025;
const DAY: usize = 05;

fn main() {
    let input = input_store::get_input(YEAR, DAY);

//     let input = r#"3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32"#;

    let (ranges_input, ids) = input.trim().split_once("\n\n").unwrap();

    let ranges = ranges_input
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.trim().split_once('-').unwrap();
            let start = start_str.parse().unwrap();
            let end = end_str.parse().unwrap();
            (start, end).into()
        })
        .collect::<Vec<Range>>();

    let ids = just_numbers(ids);

    let part_1 = ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains_value(**id)))
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
