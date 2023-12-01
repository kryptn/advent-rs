use advent::{
    input_store,
    parsers::{parse_num, parse_number_word},
};
use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    multi::many1,
    IResult,
};

const YEAR: usize = 2023;
const DAY: usize = 1;

fn maybe_parse_num(input: &str) -> IResult<&str, Option<char>> {
    if let Ok((input, value)) = alt((parse_number_word, one_of("0123456789")))(input) {
        Ok((input, Some(value)))
    } else {
        Ok((&input[1..], None))
    }
}

fn parse_line(orig: &str) -> IResult<&str, Vec<usize>> {
    let mut out = Vec::new();

    let mut input = orig;
    while input.len() > 0 {
        let (i, num) = maybe_parse_num(input)?;

        input = i;
        if let Some(num) = num {
            let d = num.to_digit(10).unwrap() as usize;
            out.push(d);
        }
    }

    // dbg!(orig, &out);

    Ok((input, out))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet"#;

    let digits: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    // dbg!(&digits);

    let mut part_1 = 0;
    for line in digits {
        let first_digit = line[0];
        let last_digit = line[line.len() - 1];
        let number = (first_digit * 10) + last_digit;
        part_1 += number;
    }
    println!("part_1 => {}", part_1);

    // let input = r#"two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen"#;

    let digits: Vec<Vec<_>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (_, parsed) = parse_line(l.trim()).unwrap();
            println!("{}: {}\n  {:?}\n\n", i, l, parsed);
            parsed
        })
        .collect();

    let mut part_2 = 0;
    for line in digits {
        let first_digit = line[0];
        let last_digit = line[line.len() - 1];
        let number = (first_digit * 10) + last_digit;
        part_2 += number;
    }
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
