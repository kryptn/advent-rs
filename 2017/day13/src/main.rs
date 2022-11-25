use std::collections::HashMap;

use advent::input_store;
use itertools::Itertools;

fn check_scanners(
    scanners: &Vec<(usize, usize)>,
    delay: usize,
) -> (usize, bool) {
    // println!("checking with delay {}:", delay);
    let mut caught = false;
    let sev = scanners
        .iter()
        .map(|(layer, range)| {
            let t = layer + delay;
            let cycle = range + range - 2;
            let pos = t % cycle;

            let v = if pos == 0 {
                caught = true;
                layer * range
            } else {
                0
            };
            // println!("  at t: {:?}, layer: {:?}, pos: {:?}, range: {:?}, cycle: {:?}, sev: {:?}", t, layer, pos, range, cycle, v);
            v
        })
        .sum();
    (sev, caught)
}

fn main() {
    let input = input_store::get_input(2017, 13);
    // let input = r#"0: 3
    // 1: 2
    // 4: 4
    // 6: 4"#;

    let scanners: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(": ")
                .map(|d| d.parse::<usize>().unwrap())
                .collect()
        })
        .map(|line: Vec<usize>| (line[0], line[1]))
        .collect();

    let (sev, _) = check_scanners(&scanners, 0, 0);
    println!("part_1 => {}", sev);

    for delay in 0.. {
        let (_, caught) = check_scanners(&scanners, delay, 1);
        if !caught {
            println!("part_2 => {}", delay);
            break;
        }
    }
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
