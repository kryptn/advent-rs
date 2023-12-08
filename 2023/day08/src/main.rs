use std::collections::HashMap;

use advent::input_store;

const YEAR: usize = 2023;
const DAY: usize = 8;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)"#;

    let input = input.trim().split("\n\n").collect::<Vec<_>>();
    let directions = input.first().unwrap().trim();
    let nodes = input
        .last()
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let l = l
                .trim()
                .replace("(", "")
                .replace(")", "")
                .replace(",", "")
                .replace("=", "");
            let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
            (
                parts[0].to_string(),
                (parts[1].to_string(), parts[2].to_string()),
            )
        })
        .collect::<HashMap<String, (String, String)>>();
    // dbg!(&nodes);
    let mut current = "AAA".to_string();
    let mut steps = 0;
    for direction in directions.chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        println!("current: {}, next_direction: {}", current, direction);
        let (left, right) = nodes.get(&current).unwrap();
        println!(
            "current: {}, next_direction: {}, ({}, {})",
            current, direction, left, right
        );
        let next = match direction {
            'L' => left,
            'R' => right,
            _ => panic!("unknown direction"),
        };
        current = next.clone();
        steps += 1;
    }
    println!("part_1 => {}", steps);
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
