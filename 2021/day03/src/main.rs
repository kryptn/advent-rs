use advent::input_store;

fn find_most_significant(n: isize) -> isize {
    let mut n = n;
    let mut sig = 0;
    while n > 0 {
        n = n >> 1;
        sig += 1;
    }
    sig
}

enum IdentifyKind {
    Most,
    Least,
}

fn identify(kind: IdentifyKind, numbers: Vec<&isize>, mask: usize) -> isize {
    let mask = mask as isize;
    let mut balance = 0;
    for number in numbers {
        balance += (*number & mask) * 2 - 1;
    }

    match kind {
        IdentifyKind::Most => {
            if balance >= 0 {
                1
            } else {
                0
            }
        }
        IdentifyKind::Least => {
            if balance <= 0 {
                0
            } else {
                1
            }
        }
    }
}

fn main() {
    let input = input_store::get_input(2021, 03);
    let numbers: Vec<isize> = input
        .trim()
        .lines()
        .map(|line| isize::from_str_radix(line.trim(), 2).unwrap())
        .collect();

    let most_significant = numbers
        .iter()
        .map(|&n| find_most_significant(n))
        .max()
        .unwrap();

    // dbg!(numbers);

    dbg!(most_significant);

    println!("part_1 => {}", "not done");
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
