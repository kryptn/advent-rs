use advent::input_store;
use num::Integer;
use std::collections::HashMap;

const YEAR: usize = 2023;
const DAY: usize = 8;

fn follow(start: &String, dir: char, nodes: &HashMap<String, (String, String)>) -> String {
    let (left, right) = nodes.get(start).unwrap();
    match dir {
        'L' => left.clone(),
        'R' => right.clone(),
        _ => panic!("unknown direction"),
    }
}

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let input = input
        .replace("(", "")
        .replace(")", "")
        .replace(",", "")
        .replace("=", "");
    let sections = input.trim().split("\n\n").collect::<Vec<_>>();
    let directions = sections.first().unwrap().trim();
    let nodes = sections
        .last()
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let parts = l.trim().split_ascii_whitespace().collect::<Vec<_>>();
            (
                parts[0].to_string(),
                (parts[1].to_string(), parts[2].to_string()),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    (directions.to_string(), nodes)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)"#;

    let (directions, nodes) = parse_input(&input);

    let mut current = "AAA".to_string();
    let mut steps = 0;
    for direction in directions.chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        let (left, right) = nodes.get(&current).unwrap();

        let next = match direction {
            'L' => left,
            'R' => right,
            _ => panic!("unknown direction"),
        };
        current = next.clone();
        steps += 1;
    }
    println!("part_1 => {}", steps);

    // let input = r#"LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)
    // "#;

    // let (directions, nodes) = parse_input(&input);

    let keys: HashMap<String, (usize, usize)> = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| {
            let mut first_found_idx = 0;
            let mut last_found_idx = 0;
            let mut last_found_delta = 0;

            let mut node = k.clone();

            for (idx, dir) in directions.chars().cycle().enumerate() {
                let next = follow(&node, dir, &nodes);
                if next.ends_with("Z") {
                    if first_found_idx == 0 {
                        first_found_idx = idx;
                    }
                    let delta = idx - last_found_idx;
                    last_found_idx = idx;

                    if delta == last_found_delta {
                        break;
                    } else {
                        last_found_delta = delta;
                    }
                }
                node = next;
            }
            (k.clone(), (first_found_idx, last_found_delta))
        })
        .collect();

    let lcm = keys
        .values()
        .map(|(_, delta)| delta)
        .fold(1, |acc, delta| delta.lcm(&acc));

    println!("part_2 => {}", lcm);
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
