use advent::input_store;
use itertools::Itertools;

fn main() {
    let input = input_store::get_input(2022, 01);

    let elves: Vec<u32> = input
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(|line| line.trim().parse::<u32>().unwrap())
                .sum()
        })
        .sorted()
        .rev()
        .collect();

    println!("part_1 => {}", elves[0]);
    println!("part_2 => {}", elves[0..3].iter().sum::<u32>());
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
