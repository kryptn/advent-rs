use std::collections::HashSet;

use advent::input_store;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Signal {
    fn from(c: char) -> Self {
        match c {
            'a' | 'A' => Signal::A,
            'b' | 'B' => Signal::B,
            'c' | 'C' => Signal::C,
            'd' | 'D' => Signal::D,
            'e' | 'E' => Signal::E,
            'f' | 'F' => Signal::F,
            'g' | 'G' => Signal::G,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pattern {
    values: HashSet<Signal>,
}

impl From<&str> for Pattern {
    fn from(input: &str) -> Pattern {
        let mut values = HashSet::new();

        for c in input.trim().chars() {
            values.insert(c.into());
        }

        Self { values }
    }
}

#[derive(Debug)]
struct Display {
    input: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl From<&str> for Display {
    fn from(input_str: &str) -> Self {
        let mut input = Vec::new();
        let mut output = Vec::new();

        let mut seen = false;

        for pattern in input_str.trim().split(" ") {
            if pattern == "|" {
                seen = true;
                continue;
            }

            if !seen {
                input.push(pattern.into());
            } else {
                output.push(pattern.into());
            }
        }

        Self { input, output }
    }
}

fn main() {
    let input = input_store::get_input(2021, 08);
    //     let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#.to_string();

    let displays: Vec<Display> = input.trim().lines().map(|l| l.into()).collect();

    let uniques: usize = displays
        .iter()
        .map(|d| {
            d.output
                .iter()
                .filter(|&p| {
                    let l = p.values.len();
                    let is_unique = l == 2 || l == 4 || l == 3 || l == 7;
                    println!("{:?} == {}", p, is_unique);

                    is_unique
                })
                .count()
        })
        .sum();

    println!("part_1 => {:?}", uniques);
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
