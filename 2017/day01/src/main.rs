use advent::input_store;

fn reverse_captcha(input: &str, offset: usize) -> u32 {
    let mut total = 0;
    let input: Vec<u32> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    let input_len = input.len();
    for i in 0..input_len {
        let this = input[i];
        let next = input[(i + offset) % input_len];
        println!("this: {}, next: {}", this, next);
        if this == next {
            total += this
        }
    }

    total
}

fn main() {
    let input = input_store::get_input(2017, 01);

    let total = reverse_captcha(&input, 1);
    println!("part_1 => {}", total);

    let total = reverse_captcha(&input, input.len() / 2);
    println!("part_2 => {}", total);
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
    #[case("1122", 3)]
    #[case("1111", 4)]
    #[case("1234", 0)]
    #[case("91212129", 9)]
    fn p1_tests(#[case] given: &str, #[case] expected: u32) {
        assert_eq!(reverse_captcha(given, 1), expected);
    }

    #[rstest]
    #[case("1212", 6)]
    #[case("1221", 0)]
    #[case("123425", 4)]
    #[case("123123", 12)]
    #[case("12131415", 4)]
    fn p2_tests(#[case] given: &str, #[case] expected: u32) {
        assert_eq!(reverse_captcha(given, (given.len() / 2)), expected);
    }
}
