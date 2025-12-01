use std::collections::HashSet;

use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 19;

fn filter_valid(valid: &Vec<&str>, input: &str, indent: usize) -> bool {
    if input.is_empty() {
        return true;
    }

    for substr in valid {
        if input.starts_with(substr) {
            // println!("{}input: {}, substr: {}", " ".repeat(indent), input, substr);
            if filter_valid(valid, &input[substr.len()..], indent + 1) {
                return true;
            }
        }
    }

    false
}

fn filter_valid_bfs<'a>(input: &'a str, valid: &'a Vec<&str>) -> bool {
    let mut queue = vec![(input, 0)];

    while !queue.is_empty() {
        // queue.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
        let (node, iter) = queue.pop().unwrap();
        // println!("{}node: {}", " ".repeat(iter), node);

        for substr in valid {
            if node.starts_with(*substr) {
                // println!("{}--substr: {}", " ".repeat(iter), substr);
                let next = &node[substr.len()..];
                if next.is_empty() {
                    return true;
                } else {
                    queue.push((next, iter + 1));
                }
            }
        }
    }

    false
}

fn build_impossible<'a>(colors: Vec<char>, valid: &'a Vec<&str>, up_to: usize) -> Vec<String> {
    let mut impossible = vec![];

    for i in 1..=up_to {
        for p in colors.iter().permutations(i) {
            let s = p.iter().map(|c| **c).collect::<String>();
            if valid.contains(&s.as_str()) || filter_valid_bfs(&s, valid) {
                continue;
            }
            impossible.push(s);
        }
    }

    impossible
}

fn impossible_splits(input: &str, impossible: &Vec<String>) -> Vec<(String, String)> {
    let mut out = vec![];

    for imp in impossible {
        let places_with_imp = input.match_indices(imp);

        for (i, _) in places_with_imp {
            let (left, right) = input.split_at(i);
            let right = &right[imp.len()..];
            out.push((left.to_string(), right.to_string()))
        }

        if input.contains(imp) {
            let (left, right) = input.split_at(input.find(imp).unwrap());
        }
    }

    out
}

fn next_matches<'a>(input: &'a str, valid: &HashSet<String>) -> Vec<String> {
    valid
        .iter()
        .filter(|v| input.starts_with(*v))
        .cloned()
        .collect()
}

fn handle<'a>(input: &'a str, valid: &mut HashSet<String>) -> bool {
    let mut queue = next_matches(input, valid);

    while let Some(node) = queue.pop() {
        let matches = next_matches(&input[node.len()..], valid);
        if matches.is_empty() {
            return false;
        }
        for m in matches {
            if m == input {
                return true;
            }
            println!("adding {}{} to valid", node, m);
            valid.insert(format!("{}{}", node, m));
            queue.push(format!("{}{}", node, m));
        }
    }
    false
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let valid_colors = vec!['r', 'g', 'b', 'w', 'u'];

    let input = r#"r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb"#;

    let mut input = input.lines();
    let mut valid: HashSet<String> = input
        .next()
        .unwrap()
        .trim()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let max_len = valid.iter().map(|s| s.len()).max().unwrap();

    // let impossible = build_impossible(valid_colors, &mut valid, max_len);

    // println!("{:?}", impossible);

    input.next().unwrap();

    println!("{:?}", valid);

    let strings = input.collect::<Vec<_>>();
    // let strings = vec![strings[0]];

    // let splits = impossible_splits(strings[0], &impossible);

    for s in &strings {
        println!("{:?}", s);
    }

    // println!("{:?}", splits);

    // let other_part_1 = strings
    //     .iter()
    //     .filter(|s| !impossible.iter().any(|imp| s.contains(imp)))
    //     .count();

    // let part_1 = strings
    //     // .iter()
    //     .par_iter()
    //     .filter(|s| filter_valid_bfs(s.trim(), &valid))
    //     .count();

    let part_1 = strings
        .iter()
        .filter(|s| handle(s.trim(), &mut valid))
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
