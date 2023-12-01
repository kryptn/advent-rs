use std::{fmt::Debug, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{opt, value},
    error::ParseError,
    sequence::delimited,
    IResult, branch::alt,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
// from https://github.com/Geal/nom/blob/master/doc/nom_recipes.md
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (input, sign) = opt(one_of("+-"))(input)?;
    let (input, num) = digit1(input)?;

    let num = {
        let n: isize = num.parse().unwrap();
        match sign {
            Some('-') => n * -1,
            _ => n,
        }
    };

    Ok((input, num))
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = digit1(input)?;
    Ok((input, num.parse().unwrap()))
}

pub fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let (input, num) = digit1(input)?;
    let num = num.parse::<T>().unwrap();
    Ok((input, num))
}

pub fn parse_coordinate(input: &str) -> IResult<&str, crate::grid::Coordinate> {
    let (input, left) = ws(parse_num)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, right) = ws(parse_num)(input)?;

    Ok((input, crate::grid::Coordinate::new(left, right)))
}

pub fn parse_number_word(input: &str) -> IResult<&str, char> {
    let (input, num) = alt((tag("zero"), tag("one"), tag("two"), tag("three"), tag("four"), tag("five"), tag("six"), tag("seven"), tag("eight"), tag("nine")))(input)?;
    let num = match num {
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("unknown number word: {}", num),
    };

    Ok((input, num))
}

pub fn parse_number_word_reversed(input: &str) -> IResult<&str, char> {
    let (input, num) = alt((tag("orez"), tag("eno"), tag("owt"), tag("eerht"), tag("ruof"), tag("evif"), tag("xis"), tag("neves"), tag("thgie"), tag("enin")))(input)?;
    let num = match num {
        "orez" => '0',
        "eno" => '1',
        "owt" => '2',
        "eerht" => '3',
        "ruof" => '4',
        "evif" => '5',
        "xis" => '6',
        "neves" => '7',
        "thgie" => '8',
        "enin" => '9',
        _ => panic!("unknown number word: {}", num),
    };

    Ok((input, num))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("0", 0)]
    #[case("1", 1)]
    #[case("-1", -1)]
    #[case("+1", 1)]
    #[case("100", 100)]
    #[case("-100", -100)]
    fn test_parse_isize(#[case] given: &str, #[case] expected: isize) {
        let (_, parsed) = parse_isize(given).unwrap();
        assert_eq!(parsed, expected);
    }

    #[rstest]
    #[case("0", 0)]
    #[case("1", 1)]
    #[case("100", 100)]
    fn test_parse_usize(#[case] given: &str, #[case] expected: isize) {
        let (_, parsed) = parse_isize(given).unwrap();
        assert_eq!(parsed, expected);
    }
}
