use std::{fmt::Debug, str::FromStr};

use nom::{
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{opt, value},
    error::ParseError,
    sequence::delimited,
    IResult,
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