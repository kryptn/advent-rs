use std::collections::HashMap;

use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 05;

fn validate(
    before: &HashMap<u32, Vec<u32>>,
    after: &HashMap<u32, Vec<u32>>,
    updates: &Vec<u32>,
) -> bool {
    let mut valid = true;

    for i in 0..updates.len() - 1 {
        let this = updates[i];
        let before_this = &updates[0..i];
        let after_this = &updates[i + 1..];

        // println!("update: {:?}", updates);
        // println!("{:?}, {:?}, {:?}", before_this, this, after_this);

        // dbg!(&this, &before_this, &after_this);

        for b in before_this {
            // assume complete

            if let Some(bl) = before.get(b) {
                if !bl.contains(&this) {
                    return false;
                }
            }
        }

        for a in after_this {
            if let Some(al) = after.get(a) {
                if !al.contains(&this) {
                    return false;
                }
            }
        }
    }

    true
}

fn get_middle(updates: &Vec<u32>) -> u32 {
    let middle_idx = updates.len() / 2;

    updates[middle_idx]
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
//     let input = r#"47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13

// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47"#;

    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut before = HashMap::new();
    let mut after = HashMap::new();

    let rules: Vec<(u32, u32)> = input[0]
        .trim()
        .lines()
        .map(|l| {
            let parts = l.split("|").collect::<Vec<&str>>();
            let x = parts[0].trim().parse().unwrap();
            let y = parts[1].trim().parse().unwrap();

            before.entry(x).or_insert(vec![]).push(y);
            after.entry(y).or_insert(vec![]).push(x);

            (x, y)
        })
        .collect();

    // dbg!(&rules, &before, &after);

    let updates: Vec<Vec<u32>> = input[1]
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // dbg!(&updates);

    let part_1: u32 = updates
        .iter()
        .filter_map(|u| {
            if validate(&before, &after, u) {
                Some(get_middle(u))
            } else {
                None
            }
        })
        .sum();

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
