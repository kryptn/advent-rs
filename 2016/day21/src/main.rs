use advent::{
    input_store,
    parsers::{parse_usize, ws},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::opt,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Rotate(Direction, usize),
    RotateOnLetter(char),
    ReverseSubset(usize, usize),
    Move(usize, usize),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let (_, inst) = parse_instruction(line).unwrap();
        inst
    }
}

fn parse_swap_position(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("swap position")(input)?;
    let (input, x) = ws(parse_usize)(input)?;
    let (input, _) = tag("with position")(input)?;
    let (input, y) = ws(parse_usize)(input)?;

    Ok((input, Instruction::SwapPosition(x, y)))
}
fn parse_swap_letter(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("swap letter")(input)?;
    let (input, x) = ws(alpha1)(input)?;
    let (input, _) = tag("with letter")(input)?;
    let (input, y) = ws(alpha1)(input)?;

    let x = x.chars().collect::<Vec<char>>().get(0).unwrap().clone();
    let y = y.chars().collect::<Vec<char>>().get(0).unwrap().clone();
    Ok((input, Instruction::SwapLetter(x, y)))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((ws(tag("left")), ws(tag("right"))))(input)?;

    let direction = match direction {
        "left" => Direction::Left,
        "right" => Direction::Right,
        _ => unreachable!(),
    };

    Ok((input, direction))
}
fn parse_rotate(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("rotate")(input)?;
    let (input, direction) = parse_direction(input)?;
    let (input, x) = ws(parse_usize)(input)?;
    let (input, _) = ws(tag("step"))(input)?;
    let (input, _) = opt(ws(tag("s")))(input)?;

    Ok((input, Instruction::Rotate(direction, x)))
}
fn parse_rotate_on_letter(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("rotate based on position of letter")(input)?;
    let (input, x) = ws(alpha1)(input)?;
    let x = x.chars().collect::<Vec<char>>().get(0).unwrap().clone();
    Ok((input, Instruction::RotateOnLetter(x)))
}
fn parse_reverse_subset(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("reverse positions")(input)?;
    let (input, x) = ws(parse_usize)(input)?;
    let (input, _) = tag("through")(input)?;
    let (input, y) = ws(parse_usize)(input)?;
    Ok((input, Instruction::ReverseSubset(x, y)))
}
fn parse_move(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move position")(input)?;
    let (input, x) = ws(parse_usize)(input)?;
    let (input, _) = tag("to position")(input)?;
    let (input, y) = ws(parse_usize)(input)?;
    Ok((input, Instruction::Move(x, y)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_swap_position,
        parse_swap_letter,
        parse_rotate,
        parse_rotate_on_letter,
        parse_reverse_subset,
        parse_move,
    ))(input)
}

fn main() {
    let input = input_store::get_input(2016, 21);

    let instructions: Vec<Instruction> = input.trim().lines().map(|l| l.into()).collect();

    dbg!(instructions);
}

fn apply_inst(chars: Vec<char>, inst: Instruction) -> Vec<char> {

}

struct Scrambler {
    original: String,
    chars: Vec<char>,
}

impl<'a> Scrambler {
    fn new(password: String) -> Self {
        let chars = password.chars().collect();
        Self {
            original: password,
            chars,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {

    }
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
