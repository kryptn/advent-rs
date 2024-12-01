use std::collections::HashMap;

use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 01;

fn main() {
    // each line is a string like "1   2"
    // split them and cast them to ints
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3
    // "#;

    let mut left = vec![];
    let mut right = vec![];
    let mut counts: HashMap<i32, i32> = HashMap::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: i32 = parts[0].parse().unwrap();
        let b: i32 = parts[1].parse().unwrap();
        left.push(a);
        right.push(b);
        counts.entry(b).and_modify(|e| *e += 1).or_insert(1);
    });

    left.sort();
    right.sort();

    let deltas = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (b - a).abs())
        .collect::<Vec<i32>>();

    let part_1 = deltas.iter().sum::<i32>();

    println!("part_1 => {}", part_1);

    let part_2 = left
        .iter()
        .map(|l| l * counts.get(l).unwrap_or(&0))
        .sum::<i32>();
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
