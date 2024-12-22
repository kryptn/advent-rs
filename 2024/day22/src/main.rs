use std::collections::HashMap;

use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 22;

fn prune(n: usize) -> usize {
    n % 16777216
}

fn mix(given: usize, secret: usize) -> usize {
    given ^ secret
}

fn price(secret: usize) -> i8 {
    (secret % 10) as i8
}

fn secret_number(secret: usize) -> usize {
    let mut n = secret;
    n = prune(mix(n * 64, n));
    n = prune(mix(n / 32, n));
    prune(mix(n * 2048, n))
}

fn crunch(secret: usize, k: usize, cache: &mut HashMap<usize, usize>) -> usize {
    let mut this: usize = secret;
    for _ in 0..k {
        this = *cache.entry(this).or_insert(secret_number(this));
    }
    this
}

fn sequence(
    secret: usize,
    k: usize,
    cache: &mut HashMap<usize, usize>,
) -> HashMap<(i8, i8, i8, i8), usize> {
    let mut out = HashMap::new();

    let changes = std::iter::successors(Some(secret), |s| {
        Some(*cache.entry(*s).or_insert(secret_number(*s)))
    })
    .tuple_windows()
    .map(|(a, b, c, d, e)| {
        let a = price(a);
        let b = price(b);
        let c = price(c);
        let d = price(d);
        let e = price(e);

        let key = (b - a, c - b, d - c, e - d);

        (key, e as usize)
    });

    for (k, e) in changes.take(k) {
        out.entry(k).or_insert(e);
    }

    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = "1\n2\n3\n2024";

    let secrets = input
        .trim()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let part_1 = secrets
        .iter()
        .map(|s| crunch(*s, 2000, &mut cache))
        .sum::<usize>();

    println!("part_1 => {}", part_1);

    let mut sequences = HashMap::new();
    secrets.iter().for_each(|s| {
        for (k, e) in sequence(*s, 2000, &mut cache) {
            sequences.entry(k).and_modify(|v| *v += e).or_insert(e);
        }
    });

    let part_2 = sequences.values().max().unwrap();

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

    // 15887950
    // 16495136
    // 527345
    // 704524
    // 1553684
    // 12683156
    // 11100544
    // 12249484
    // 7753432
    // 5908254

    #[rstest]
    #[case(123, 1, 15887950)]
    #[case(123, 2, 16495136)]
    #[case(123, 3, 527345)]
    #[case(123, 4, 704524)]
    #[case(123, 5, 1553684)]
    #[case(123, 6, 12683156)]
    #[case(123, 7, 11100544)]
    #[case(123, 8, 12249484)]
    #[case(123, 9, 7753432)]
    #[case(123, 10, 5908254)]
    fn p1_tests(#[case] secret: usize, #[case] k: usize, #[case] expected: usize) {
        let mut cache = HashMap::new();
        assert_eq!(crunch(secret, k, &mut cache), expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
