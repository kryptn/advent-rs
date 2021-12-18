use std::rc::Rc;

use advent::{input_store, parsers::parse_usize};
use nom::{bytes::complete::tag, combinator::peek, IResult};

#[derive(Debug, Clone)]
enum Kind {
    Pair(Rc<Pair>),
    Value(usize),
}

#[derive(Debug, Clone)]
struct Pair {
    left: Kind,
    right: Kind,
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
    let pairs: Vec<Pair> = input
        .trim()
        .lines()
        .map(|line| {
            let pair = parse_pair(line.trim()).expect("known good").1;
            pair
        })
        .collect();

    dbg!(pairs);

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
