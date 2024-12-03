use advent::{input_store, parsers::parse_coordinate};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{anychar, char, digit0, digit1},
    combinator::{map_res, verify},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

const YEAR: usize = 2024;
const DAY: usize = 03;

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    // println!("parsing {}", input);
    let (input, result) = delimited(
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
    )(input)?;

    Ok((input, Instruction::Mul(result.0, result.1)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, inst) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, inst) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_until_inst(input: &str) -> IResult<&str, (Vec<char>, Instruction)> {
    many_till(anychar, alt((parse_mul, parse_do, parse_dont)))(input)
}

fn parse_multiple_inst(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, results) = many0(parse_until_inst)(input)?;

    let pairs = results.into_iter().map(|(_, pair)| pair).collect();

    Ok((input, pairs))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let instructions = parse_multiple_inst(&input).unwrap().1;

    let part_1 = instructions
        .iter()
        .map(|inst| match inst {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<u32>();

    println!("part_1 => {}", part_1);

    let mut take = true;
    let mut part_2 = 0;

    for inst in instructions {
        match (inst, take) {
            (Instruction::Do, _) => take = true,
            (Instruction::Dont, _) => take = false,
            (Instruction::Mul(a, b), true) => part_2 += a * b,
            (Instruction::Mul(_, _), false) => {}
        }
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
