use advent::input_store;
use itertools::Itertools;

#[derive(Copy, Clone)]

struct Symbol(isize);

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '2' => Symbol(2),
            '1' => Symbol(1),
            '0' => Symbol(0),
            '-' => Symbol(-1),
            '=' => Symbol(-2),
            _ => panic!(),
        }
    }
}

impl Symbol {
    fn decr(&mut self) {
        self.0 -= 1;
    }

    fn incr(&mut self) {
        self.0 += 1;
    }
}
const CHARS: [&str; 5] = ["=", "-", "0", "1", "2"];

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = (self.0 + 2) as usize;
        write!(f, "{}", CHARS[idx])
    }
}

#[derive(Clone)]
struct Number(Vec<Symbol>);

impl Number {
    fn to_isize(&self) -> isize {
        let mut value = 0;
        for (i, sym) in self.0.iter().rev().enumerate() {
            value += sym.0 * 5isize.pow(i as u32);
        }

        value
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chs = self.0.iter().join("");
        write!(f, "{chs}")
    }
}

impl From<&str> for Number {
    fn from(value: &str) -> Self {
        let out = value.trim().chars().map(|c| c.into()).collect();
        Self(out)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        let mut ceiling = 0;
        let mut number = Number(Vec::new());
        number.0.push(Symbol(2));
        while 2 * 5isize.pow(ceiling) < value {
            ceiling += 1;
            number.0.push(Symbol(2));
        }

        for idx in 0..number.0.len() {
            while number.to_isize() >= value {
                number.0[idx].decr();
            }
            number.0[idx].incr();
        }

        number
    }
}

fn main() {
    let input = input_store::get_input(2022, 25);
    // let input = r#"1=-0-2
    // 12111
    // 2=0=
    // 21
    // 2=01
    // 111
    // 20012
    // 112
    // 1=-1=
    // 1-12
    // 12
    // 1=
    // 122"#;

    let numbers: Vec<Number> = input
        .trim()
        .lines()
        .map(|line| Number::from(line))
        .collect();

    let p1_decimal: isize = numbers.into_iter().map(|n| n.to_isize()).sum();
    let p1_number = Number::from(p1_decimal);

    println!("part_1 => {}", p1_number);
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
    #[case("1=-0-2", 1747)]
    #[case("12111 ", 906)]
    #[case("2=0=  ", 198)]
    #[case("21    ", 11)]
    #[case("2=01  ", 201)]
    #[case("111   ", 31)]
    #[case("20012 ", 1257)]
    #[case("112   ", 32)]
    #[case("1=-1= ", 353)]
    #[case("1-12  ", 107)]
    #[case("12    ", 7)]
    #[case("1=    ", 3)]
    #[case("122   ", 37)]
    fn from_snafu_to_isize(#[case] given: &str, #[case] expected: isize) {
        let number = Number::from(given);
        assert_eq!(number.to_isize(), expected);
    }

    #[rstest]
    #[case(1, "1")]
    #[case(2, "2")]
    #[case(3, "1=")]
    #[case(4, "1-")]
    #[case(5, "10")]
    #[case(6, "11")]
    #[case(7, "12")]
    #[case(8, "2=")]
    #[case(9, "2-")]
    #[case(10, "20")]
    #[case(15, "1=0")]
    #[case(20, "1-0")]
    #[case(2022, "1=11-2")]
    #[case(12345, "1-0---0")]
    #[case(314159265, "1121-1110-1=0")]
    fn from_isize_to_snafu(#[case] given: isize, #[case] expected: &str) {
        let number = Number::from(given);
        let repr = number.to_string();
        assert_eq!(&repr, expected);
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
