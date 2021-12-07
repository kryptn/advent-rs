use std::collections::HashMap;

use advent::input_store;

fn main() {
    let input = input_store::get_input(2021, 07);
    //let input = "16,1,2,0,4,2,7,1,2,14";

    let positions: Vec<i64> = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let min_p = positions.iter().min().unwrap().clone();
    let max_p = positions.iter().max().unwrap().clone();
    let mut fuel = i64::MAX;

    for target in min_p..=max_p {
        let this_fuel = positions.iter().map(|p| (target - p).abs()).sum();
        if this_fuel < fuel {
            fuel = this_fuel;
        }
    }

    println!("part_1 => {}", fuel);

    let mut costs: HashMap<i64, i64> = HashMap::new();
    let mut fuel = i64::MAX;

    for target in min_p..=max_p {
        let this_fuel = positions
            .iter()
            .map(|p| {
                let distance = (target - p).abs();
                if !costs.contains_key(&distance) {
                    let cost = (1..=distance).reduce(|a, b| a + b).unwrap_or_default();
                    costs.insert(distance, cost);
                }
                costs.get(&distance).unwrap().clone()
            })
            .sum();

        if this_fuel < fuel {
            fuel = this_fuel;
        }
    }

    println!("part_2 => {}", fuel);
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

    #[test]
    fn trything() {
        assert_eq!((1..5).reduce(|a, b| a + b).unwrap(), 10);
        assert_eq!((1..(16 - 5)).reduce(|a, b| a + b).unwrap(), 66);
        assert_eq!((1..(2 as isize - 5).abs()).reduce(|a, b| a + b).unwrap(), 6);
    }
}
