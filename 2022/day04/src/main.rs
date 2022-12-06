use advent::input_store;

struct Pair {
    a: (usize, usize),
    b: (usize, usize),
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let mut two = input.trim().split(",");

        let mut a_raw = two.next().unwrap().split("-");
        let a1 = a_raw.next().unwrap().parse().unwrap();
        let a2 = a_raw.next().unwrap().parse().unwrap();

        let mut b_raw = two.next().unwrap().split("-");
        let b1 = b_raw.next().unwrap().parse().unwrap();
        let b2 = b_raw.next().unwrap().parse().unwrap();

        let a = (a1, a2);
        let b = (b1, b2);

        if a1 < b1 {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 04);
    let pairs: Vec<Pair> = input.trim().lines().map(|line| line.into()).collect();

    let part_1 = pairs
        .iter()
        .filter(|pair| {
            pair.b.0 >= pair.a.0 && pair.b.1 <= pair.a.1
                || pair.a.0 >= pair.b.0 && pair.a.1 <= pair.b.1
        })
        .count();
    println!("part_1 => {}", part_1);

    let part_2 = pairs.iter().filter(|pair| pair.b.0 <= pair.a.1).count();
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
