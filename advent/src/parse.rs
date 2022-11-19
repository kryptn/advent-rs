use nom::{character::complete::digit1, IResult};

trait Parsable {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parsable for String {
    fn parse(input: &str) -> IResult<&str, Self> {
        Ok(("", input.to_string()))
    }
}

impl Parsable for i32 {
    fn parse(input: &str) -> IResult<&str, Self> {
        Ok(("", input.parse().unwrap()))
    }
}

impl Parsable for u32 {
    fn parse(input: &str) -> IResult<&str, Self> {
        Ok(("", input.parse().unwrap()))
    }
}

impl Parsable for isize {
    fn parse(input: &str) -> IResult<&str, Self> {
        Ok(("", input.parse().unwrap()))
    }
}

impl Parsable for usize {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, num) = digit1(input)?;
        Ok((input, num.parse().unwrap()))
    }
}

fn parse<T1>(input: &str) -> T1
where
    T1: Parsable,
{
    let (_, obj) = T1::parse(input).unwrap();
    obj
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        let orig = "abcd";
        let expected = "abcd".to_string();

        let parsed: String = parse(orig);

        assert_eq!(expected, parsed);
    }
}
