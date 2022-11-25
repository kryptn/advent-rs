use std::collections::{HashMap, HashSet};

use advent::input_store;

#[derive(Debug)]
struct Disc {
    name: String,
    supporting: Vec<String>,
    weight: usize,
    supporting_weight: usize,
}

impl From<&str> for Disc {
    fn from(input: &str) -> Self {
        let input = input
            .replace("(", "")
            .replace(")", "")
            .replace("->", "")
            .replace(",", "");
        let mut parts = input.split_whitespace();

        let name = parts.next().unwrap().to_string();
        let weight: usize = parts.next().unwrap().parse().unwrap();
        let supporting = parts.map(|n| n.to_string()).collect();

        Self {
            name,
            supporting,
            weight,
            supporting_weight: weight,
        }
    }
}

fn weigh_supported(key: String, discs: &mut HashMap<String, Disc>) -> usize {
    let (needs_calculated, supporting) = {
        let disc = discs.get(&key).unwrap();
        (
            !disc.supporting.is_empty() && disc.weight == disc.supporting_weight,
            disc.supporting.clone(),
        )
    };

    if needs_calculated {
        let weight: usize = supporting
            .iter()
            .map(|name| weigh_supported(name.clone(), discs))
            .sum();
        let dm = discs.get_mut(&key).unwrap();
        dm.supporting_weight = weight + dm.weight;
    }

    discs.get(&key).unwrap().supporting_weight
}

fn is_unstable(key: String, discs: &HashMap<String, Disc>) -> bool {
    let disc = discs.get(&key).unwrap();
    if disc.supporting.is_empty() {
        return false;
    }

    let mut supported_weights = HashMap::new();
    for supported in disc.supporting.iter() {
        let d = discs.get(supported).unwrap().supporting_weight;
        *supported_weights.entry(d).or_insert(0) += d;
    }

    supported_weights.len() > 1
}

fn traverse_unstable(key: String, discs: &HashMap<String, Disc>) {
    let mut key = key;

    loop {
        let disc = discs.get(&key).unwrap();
        let unstable_sub = disc
            .supporting
            .iter()
            .cloned()
            .filter(|a| is_unstable(a.clone(), discs))
            .next();

        println!("this: {}: {}\nsupports:", disc.name, disc.weight);
        for supporting in disc.supporting.iter() {
            let sub = discs.get(supporting).unwrap();
            println!(
                "    {}: {}, {}",
                sub.name, sub.weight, sub.supporting_weight
            );
        }

        println!("\n\n");

        // dbg!(disc);
        if let Some(sub) = unstable_sub {
            key = sub.clone();
        } else {
            break;
        }
    }
}

fn main() {
    let input = input_store::get_input(2017, 07);

    let mut tree = HashMap::new();
    let mut discs = HashMap::new();

    for line in input.lines() {
        let disc: Disc = line.into();
        tree.insert(disc.name.clone(), disc.supporting.clone());
        discs.insert(disc.name.clone(), disc);
    }

    let supporting: HashSet<String> = tree.keys().cloned().collect();
    let supported: HashSet<String> = tree.values().flatten().cloned().collect();

    let base = supporting.difference(&supported).next().unwrap();

    println!("part_1 => {}", base);

    weigh_supported(base.clone(), &mut discs);

    // dbg!(&discs);

    traverse_unstable(base.clone(), &discs);

    // for (name, disc) in discs.iter() {
    //     println!("disc: {}, unstable: {}", name, is_unstable(name.clone(), &discs))
    // }

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
