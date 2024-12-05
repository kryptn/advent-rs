use std::collections::HashMap;

use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 05;

enum UpdateStatus {
    Changed(Vec<u32>),
    Unchanged(Vec<u32>),
}

// returns None if no change necessary
fn fix_step(
    before: &HashMap<u32, Vec<u32>>,
    after: &HashMap<u32, Vec<u32>>,
    updates: Vec<u32>,
) -> UpdateStatus {
    let mut cursor = 0;
    let mut target = 0;
    let mut broken = false;

    'outer: for i in 0..updates.len() - 1 {
        cursor = 0;
        let this = updates[i];
        while cursor < updates.len() {
            if cursor < i {
                let before_this = updates[cursor];
                if let Some(bl) = before.get(&before_this) {
                    if !bl.contains(&this) {
                        target = i;
                        broken = true;
                        break 'outer;
                    }
                }
            } else if cursor > i {
                let after_this = updates[cursor];
                if let Some(al) = after.get(&after_this) {
                    if !al.contains(&this) {
                        target = i;
                        broken = true;
                        break 'outer;
                    }
                }
            }
            cursor += 1;
        }
    }

    if broken {
        let mut next = updates.clone();
        next.swap(target, cursor);

        return UpdateStatus::Changed(next);
    }
    UpdateStatus::Unchanged(updates)
}

fn fix_updatestatus_step(
    before: &HashMap<u32, Vec<u32>>,
    after: &HashMap<u32, Vec<u32>>,
    update_status: UpdateStatus,
) -> UpdateStatus {
    match update_status {
        UpdateStatus::Changed(vec) => fix_step(before, after, vec.clone()),
        UpdateStatus::Unchanged(_) => update_status,
    }
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

    input[0].trim().lines().for_each(|l| {
        let parts = l.split("|").collect::<Vec<&str>>();
        let x = parts[0].trim().parse().unwrap();
        let y = parts[1].trim().parse().unwrap();

        before.entry(x).or_insert(vec![]).push(y);
        after.entry(y).or_insert(vec![]).push(x);
    });

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
        .filter_map(|u| match fix_step(&before, &after, u.clone()) {
            UpdateStatus::Changed(_) => None,
            UpdateStatus::Unchanged(_) => Some(get_middle(u)),
        })
        .sum();

    println!("part_1 => {}", part_1);

    let mut invalid_updates: Vec<UpdateStatus> = updates
        .iter()
        .filter_map(|u| {
            let result = fix_step(&before, &after, u.clone());
            match result {
                UpdateStatus::Changed(_) => Some(result),
                UpdateStatus::Unchanged(_) => None,
            }
        })
        .collect();

    while invalid_updates.iter().any(|us| match us {
        UpdateStatus::Changed(_) => true,
        UpdateStatus::Unchanged(_) => false,
    }) {
        invalid_updates = invalid_updates
            .into_iter()
            .map(|us| fix_updatestatus_step(&before, &after, us))
            .collect();
    }

    let part_2: u32 = invalid_updates
        .iter()
        .map(|u| match u {
            UpdateStatus::Unchanged(u) => get_middle(u),
            _ => panic!("should not happen"),
        })
        .sum();

    // let part_2: u32 = updates
    //     .iter()
    //     .filter_map(|u| match fix_step(&before, &after, u) {
    //         Some(fixed) => Some(fixed),
    //         None => None,
    //     })
    //     .filter(|u| validate(&before, &after, u))
    //     .map(|u| get_middle(u))
    //     .sum();
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
