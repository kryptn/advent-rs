use advent::input_store;

fn curve(input: &str) -> String {
    let mut out = format!("{}0", input);
    for c in input.chars().rev() {
        let n = if c == '0' { "1" } else { "0" };
        out.push_str(n);
    }
    out
}

fn checksum(input: &str) -> String {
    let mut out = String::new();
    for i in (0..input.len()).step_by(2) {
        let l = input.get(i..i + 1).unwrap();
        let r = input.get(i + 1..i + 2).unwrap();
        if l == r {
            out.push_str("1");
        } else {
            out.push_str("0");
        }
    }
    if out.len() % 2 == 0 {
        out = checksum(&out);
    }

    out
}

fn bloat(input: &str, to: usize) -> String {
    let mut out = String::from(input);
    while out.len() < to {
        out = curve(&out);
    }

    String::from(out.get(0..to).unwrap())
}

fn bloat_checksum(input: &str, to: usize) -> String {
    let bloated = bloat(input, to);
    checksum(&bloated)
}

fn main() {
    let input = input_store::get_input(2016, 16).trim().to_string();
    let size = 272;

    // let input = "10000";
    // let size = 20;

    println!("part 1 => {}", bloat_checksum(&input, size));

    let size = 35651584;
    println!("part 1 => {}", bloat_checksum(&input, size));
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
    #[case("1", "100")]
    #[case("0", "001")]
    #[case("11111", "11111000000")]
    #[case("111100001010", "1111000010100101011110000")]
    fn test_curve(#[case] input: &str, #[case] expected: String) {
        assert_eq!(curve(input), expected)
    }

    #[rstest]
    #[case("11", "1")]
    #[case("00", "1")]
    #[case("10", "0")]
    #[case("110010110100", "100")]
    fn test_checksum(#[case] input: &str, #[case] expected: String) {
        assert_eq!(checksum(input), expected)
    }

    #[test]
    fn test_bloat() {
        assert_eq!(bloat("1010", 100).len(), 100);
    }

    #[rstest]
    #[case("10000", 20, "01100")]
    fn test_bloat_checksum(#[case] input: &str, #[case] to: usize, #[case] expected: String) {
        assert_eq!(bloat_checksum(input, to), expected);
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
