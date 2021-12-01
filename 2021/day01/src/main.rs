use advent::input_store;
use itertools::{self, Itertools};

fn main() {
    let input = input_store::get_input(2021, 01);

    let depths: Vec<i32> = input
        .trim()
        .lines()
        .map(|l| {
            //dbg!(l);
            l.trim().parse::<i32>().unwrap()
        })
        .collect();

    let part_1 = depths.iter().tuple_windows().filter(|(a, b)| a < b).count();
    println!("part 1 => {}", part_1);

    let part_2 = depths
        .iter()
        .tuple_windows()
        .filter(|(a, _, _, b)| a < b)
        .count();
    println!("part 2 => {}", part_2);
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
