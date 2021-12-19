use std::rc::Rc;

use advent::{input_store, parsers::parse_usize};
use nom::{bytes::complete::tag, combinator::peek, IResult};

#[derive(Debug, Clone)]
enum Kind {
    Pair(Rc<Pair>),
    Value(usize),
}

impl From<usize> for Kind {
    fn from(value: usize) -> Self {
        Self::Value(value)
    }
}

impl From<Pair> for Kind {
    fn from(pair: Pair) -> Self {
        Kind::Pair(Rc::new(pair))
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Pair(value) => write!(f, "{}", value),
            Kind::Value(pair) => write!(f, "{}", pair),
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    left: Kind,
    right: Kind,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Pair {
    fn add(self, other: Self) -> Self {
        Self {
            left: self.into(),
            right: other.into(),
        }
    }
}

fn parse_kind(input: &str) -> IResult<&str, Kind> {
    match peek(parse_usize)(input) {
        Ok(_) => {
            let (input, value) = parse_usize(input)?;
            Ok((input, Kind::Value(value)))
        }
        Err(_) => {
            let (input, pair) = parse_pair(input)?;
            Ok((input, Kind::Pair(Rc::new(pair))))
        }
    }
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, _) = tag("[")(input)?;
    let (input, left) = parse_kind(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, right) = parse_kind(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Pair { left, right }))
}

fn main() {
    let input = input_store::get_input(2021, 18);

    let input = r#"[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"#;

    let pairs: Vec<Pair> = input
        .trim()
        .lines()
        .map(|line| {
            let pair = parse_pair(line.trim()).expect("known good").1;
            pair
        })
        .collect();

    for pair in pairs {
        println!("{}", pair);
        dbg!(pair);
    }

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
