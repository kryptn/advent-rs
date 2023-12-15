use core::ascii;

use advent::input_store;

const YEAR: usize = 2023;
const DAY: usize = 15;

struct Sequence {
    raw: String,
}

impl From<&str> for Sequence {
    fn from(s: &str) -> Self {
        Self { raw: s.to_string() }
    }
}

impl Sequence {
    #![allow(arithmetic_overflow)]
    fn hasher(&self) -> usize {
        let mut out = 0;
        for c in self.raw.chars() {
            let ascii = c as usize;
            out = ((out + ascii) * 17) % 256
        }
        out as usize
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
    let part_1 = sequences.iter().map(Sequence::hasher).sum::<usize>();

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
