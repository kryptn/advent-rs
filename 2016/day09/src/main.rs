use std::{collections::HashSet, rc::Rc};

use advent::{grid, input_store};
use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete as ch,
    multi::{self, fold_many1},
    sequence, IResult,
};

fn parse_xy(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, characters) = ch::digit1(input)?;
    let (input, _) = ch::char('x')(input)?;
    let (input, times) = ch::digit1(input)?;

    Ok((input, (characters.parse().unwrap(), times.parse().unwrap())))
}

fn parse_marker(input: &str) -> IResult<&str, (usize, usize)> {
    sequence::delimited(ch::char('('), parse_xy, ch::char(')'))(input)
}

fn expand_marker(input: &str) -> IResult<&str, String> {
    let (input, (characters, times)) = parse_marker(input)?;

    let (input, text) = take(characters)(input)?;

    let mut out = String::new();
    for _ in 0..times {
        out.push_str(text.clone());
    }

    Ok((input, out))
}

fn boring(input: &str) -> IResult<&str, String> {
    let (input, out) = ch::alpha1(input)?;
    Ok((input, out.into()))
}

fn decompress(input: &str) -> IResult<&str, String> {
    fold_many1(
        alt((boring, expand_marker)),
        String::new,
        |mut acc: String, item| {
            acc.push_str(&item);
            acc
        },
    )(input)
}

fn main() {
    let input = input_store::get_input(2016, 9);

    let (_, expanded) = decompress(&input).unwrap();

    let total = expanded
        .chars()
        .fold(0, |a, c| if c == ' ' { a } else { a + 1 });

    println!("part 1 => {}", total);
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
    #[case("A(1x5)BC", "ABBBBBC")]
    #[case("(3x3)XYZ", "XYZXYZXYZ")]
    #[case("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG")]
    #[case("(6x1)(1x3)A", "(1x3)A")]
    #[case("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY")]
    fn parse_test(#[case] given: &str, #[case] expected: String) {
        let (_, expanded) = decompress(given).unwrap();
        assert_eq!(expanded, expected);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
