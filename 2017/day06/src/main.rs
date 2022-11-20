use std::collections::HashSet;

use advent::input_store;

struct Ring {
    blocks: Vec<usize>,
    cycles: usize,
    seen: HashSet<Vec<usize>>,
}

impl Ring {
    fn cycle(&mut self) {
        let banks = self.blocks.len();
        let (idx, &blocks) = self
            .blocks
            .iter()
            .enumerate()
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();

        *self.blocks.get_mut(idx).unwrap() = 0;

        for i in idx + 1..idx + 1 + blocks {
            *self.blocks.get_mut(i % banks).unwrap() += 1;
        }

        self.cycles += 1;
        self.seen.insert(self.blocks.clone());
    }
}

impl From<String> for Ring {
    fn from(input: String) -> Self {
        let blocks: Vec<usize> = input
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let cycles = 0;
        let mut seen = HashSet::new();
        seen.insert(blocks.clone());

        Self {
            blocks,
            cycles,
            seen,
        }
    }
}

impl From<Vec<usize>> for Ring {
    fn from(blocks: Vec<usize>) -> Self {
        let cycles = 0;
        let mut seen = HashSet::new();
        seen.insert(blocks.clone());

        Self {
            blocks,
            cycles,
            seen,
        }
    }
}

fn main() {
    let input = input_store::get_input(2017, 06);
    // let input = "0 2 7 0".to_string();

    let mut ring: Ring = input.into();
    while ring.cycles + 1 == ring.seen.len() {
        ring.cycle();
    }

    println!("part_1 => {}", ring.cycles);

    let mut ring2: Ring = ring.blocks.into();
    while ring2.cycles + 1 == ring2.seen.len() {
        ring2.cycle();
    }

    println!("part_2 => {}", ring2.cycles);
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
