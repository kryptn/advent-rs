use advent::{
    input_store,
    parsers::{parse_number_word, parse_number_word_reversed},
};
use nom::{branch::alt, character::complete::one_of, IResult};

const YEAR: usize = 2023;
const DAY: usize = 1;

fn maybe_parse_num(input: &str) -> IResult<&str, Option<char>> {
    if let Ok((input, value)) = alt((parse_number_word, one_of("0123456789")))(input) {
        Ok((input, Some(value)))
    } else {
        Ok((&input[1..], None))
    }
}

fn maybe_parse_num_reversed(input: &str) -> IResult<&str, Option<char>> {
    if let Ok((input, value)) = alt((parse_number_word_reversed, one_of("0123456789")))(input) {
        Ok((input, Some(value)))
    } else {
        Ok((&input[1..], None))
    }
}

fn parse_first_num(input: &str) -> IResult<&str, char> {
    let (mut input, mut value) = maybe_parse_num(input)?;
    while value.is_none() {
        (input, value) = maybe_parse_num(input)?;
    }

    Ok((input, value.unwrap()))
}

fn parse_last_num(input: &str) -> IResult<&str, char> {
    let (mut input, mut value) = maybe_parse_num_reversed(input)?;
    while value.is_none() {
        (input, value) = maybe_parse_num_reversed(input)?;
    }

    Ok((input, value.unwrap()))
}

fn parse_line(orig: &str) -> IResult<&str, usize> {
    let (_, first) = parse_first_num(orig)?;

    let reversed_line = orig.chars().rev().collect::<String>();
    let (_, last) = parse_last_num(&reversed_line).unwrap();

    let out_str = format!("{}{}", first, last);
    let out = out_str.parse::<usize>().unwrap();
    Ok(("", out))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"1abc2
    // pqr3stu8vwxca
    // a1b2c3d4e5f
    // treb7uchet"#;

    let digits: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut part_1 = 0;
    for line in digits {
        let first_digit = line[0];
        let last_digit = line[line.len() - 1];
        let number = (first_digit * 10) + last_digit;
        part_1 += number;
    }
    println!("part_1 => {}", part_1);

    let part_2: usize = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (_, parsed) = parse_line(l.trim()).unwrap();
            println!("{}: {}\n  {:?}\n\n", i, l, parsed);
            parsed
        })
        .sum();

    println!("part_2 => {}", part_2);
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
