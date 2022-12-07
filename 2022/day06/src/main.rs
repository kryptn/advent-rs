use std::collections::HashSet;

use advent::input_store;

fn first_unique_window(input: &str, window: usize) -> Option<usize> {
    for i in 0..input.len() - window {
        let set: HashSet<char> = input[i..i + window].chars().collect();
        if set.len() == window {
            return Some(i + window);
        }
    }
    None
}

fn main() {
    let input = input_store::get_input(2022, 06);

    println!("part_1 => {}", first_unique_window(&input, 4).unwrap());
    println!("part_2 => {}", first_unique_window(&input, 14).unwrap());
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
