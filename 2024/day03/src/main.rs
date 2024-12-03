use advent::{input_store, parsers::parse_coordinate};
use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::complete::{anychar, char, digit0, digit1},
    combinator::{map_res, verify},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

const YEAR: usize = 2024;
const DAY: usize = 03;

pub fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    // println!("parsing {}", input);
    delimited(
        tag("mul("),
        separated_pair(
            map_res(
                verify(digit0, |s: &str| s.len() >= 1 && s.len() <= 3),
                |s: &str| s.parse::<u32>(),
            ),
            char(','),
            map_res(
                verify(digit0, |s: &str| s.len() >= 1 && s.len() <= 3),
                |s: &str| s.parse::<u32>(),
            ),
        ),
        char(')'),
    )(input)
}

fn parse_until_mul(input: &str) -> IResult<&str, (Vec<char>, (u32, u32))> {
    many_till(anychar, parse_mul)(input)
}

pub fn parse_multiple_mul(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, results) = many0(parse_until_mul)(input)?;

    let pairs = results.into_iter().map(|(_, pair)| pair).collect();

    Ok((input, pairs))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let part_1: u32 = input
        .lines()
        .flat_map(|line| {
            let (_, pairs) = parse_multiple_mul(line).unwrap();
            pairs
        })
        .map(|(a, b)| a * b)
        .sum();

    println!("part_1 => {}", part_1);
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
