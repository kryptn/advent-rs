use std::collections::HashSet;

use advent::input_store;
use itertools::Itertools;

fn main() {
    let input = input_store::get_input(2022, 06);

    for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        let set: HashSet<char> = vec![a, b, c, d].iter().cloned().collect();
        if set.len() == 4 {
            println!("part_1 => {}", i + 4);
            break;
        }
    }

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
