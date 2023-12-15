use core::ascii;

use advent::input_store;

const YEAR: usize = 2023;
const DAY: usize = 15;

enum Oper {
    Remove(String),
    Insert(String, usize),
}

struct Sequence {
    raw: String,
    label: String,
    operation: Oper,
}

impl From<&str> for Sequence {
    fn from(s: &str) -> Self {
        let raw = s.to_string();
        let label = raw
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();
        let operation = if raw.contains("=") {
            let parts = raw.split("=").collect::<Vec<_>>();
            let focal_length = parts.last().unwrap().parse::<usize>().unwrap();
            Oper::Insert(label.clone(), focal_length)
        } else {
            Oper::Remove(label.clone())
        };

        Self {
            raw,
            label,
            operation,
        }
    }
}

fn hash(s: &str) -> usize {
    let mut out = 0;
    for c in s.chars() {
        let ascii = c as usize;
        out = ((out + ascii) * 17) % 256
    }
    out as usize
}

fn apply_seq(slots: &mut Vec<Vec<(String, usize)>>, seq: &Sequence) {
    let idx = hash(&seq.label);
    match &seq.operation {
        Oper::Remove(label) => {
            slots[idx].retain(|(l, _)| l != label);
        }
        Oper::Insert(label, focal_length) => {
            let i = slots[idx].iter().position(|(l, _)| l == label);
            match i {
                Some(i) => slots[idx][i].1 = *focal_length,
                None => slots[idx].push((label.clone(), *focal_length)),
            }
        }
    }
}

fn score(slots: &Vec<Vec<(String, usize)>>) -> usize {
    slots
        .iter()
        .enumerate()
        .map(|(i, slot)| {
            slot.iter()
                .enumerate()
                .map(move |(si, (_, focal_length))| focal_length * (i + 1) * (si + 1))
        })
        .flatten()
        .sum::<usize>()
}

fn print_slots(slots: &Vec<Vec<(String, usize)>>) {
    for (i, slot) in slots.iter().enumerate() {
        if slot.is_empty() {
            continue;
        }
        print!("Box {i}: ");
        for (_, (label, focal_length)) in slot.iter().enumerate() {
            print!("[{label} {focal_length}] ");
        }
        println!("");
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

    let sequences = input
        .trim()
        .split(",")
        .map(Sequence::from)
        .collect::<Vec<_>>();
    let part_1 = sequences.iter().map(|s| hash(&s.raw)).sum::<usize>();
    println!("part_1 => {}", part_1);

    let mut slots = vec![vec![]; 256];
    sequences.iter().for_each(|s| {
        apply_seq(&mut slots, s);
        // println!("After {} hash={}", s.raw, hash(&s.label));
        // print_slots(&slots);
        // println!("\n\n");
    });
    println!("part_2 => {}", score(&slots));
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
