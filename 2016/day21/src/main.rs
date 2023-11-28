use std::{collections::HashMap, ops::Index};

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

impl Instruction {
    fn apply(&self, s: &mut Vec<char>) {
        match self {
            Instruction::SwapPosition(x, y) => {
                s.swap(*x, *y);
            }
            Instruction::SwapLetter(x, y) => {
                let x = s.iter().position(|c| c == x).unwrap();
                let y = s.iter().position(|c| c == y).unwrap();
                s.swap(x, y);
            }
            Instruction::Rotate(direction, x) => match direction {
                Direction::Left => {
                    s.rotate_left(*x);
                }
                Direction::Right => {
                    s.rotate_right(*x);
                }
            },
            Instruction::RotateOnLetter(x) => {
                let x = s.iter().position(|c| c == x).unwrap();
                let x = if x >= 4 { x + 2 } else { x + 1 };
                let x = x % s.len();
                s.rotate_right(x);
            }
            Instruction::ReverseSubset(x, y) => {
                let mut tmp = s.clone();
                for i in *x..=*y {
                    tmp[i] = s[*y - (i - x)];
                }
                s.clone_from(&tmp);
            }
            Instruction::Move(x, y) => {
                let tmp = s.remove(*x);
                s.insert(*y, tmp);
            }
        }
    }

    fn unapply(&self, s: &mut Vec<char>) {
        match self {
            Instruction::Rotate(direction, x) => {
                let inst = match direction {
                    Direction::Left => Instruction::Rotate(Direction::Right, *x),
                    Direction::Right => Instruction::Rotate(Direction::Left, *x),
                };
                inst.apply(s);
            }
            Instruction::RotateOnLetter(x) => {
                let x = s.iter().position(|c| c == x).unwrap();
                s.rotate_left(LOOKUP_TABLE[x]);
            }
            Instruction::Move(x, y) => {
                let inst = Instruction::Move(*y, *x);
                inst.apply(s);
            }
            _ => self.apply(s),
        }
    }
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

const LOOKUP_TABLE: [usize; 8] = [1, 1, 6, 2, 7, 3, 0, 4];

fn main() {
    let input = input_store::get_input(2016, 21);

    let instructions: Vec<Instruction> = input.trim().lines().map(|l| l.into()).collect();

    dbg!(&instructions);

    let mut password: Vec<char> = "abcdefgh".to_string().chars().collect();
    for inst in &instructions {
        inst.apply(&mut password);
    }

    // let mut lookup_table = HashMap::new();

    // let pw = "abcdefgh";
    // for ch in pw.chars() {
    //     let mut password: Vec<char> = pw.to_string().chars().collect();
    //     let orig_idx = password.iter().position(|c| c == &ch).unwrap();
    //     let inst = Instruction::RotateOnLetter(ch);
    //     inst.apply(&mut password);
    //     let after_idx = password.iter().position(|c| c == &ch).unwrap();
    //     let rotation = (after_idx+pw.len() - orig_idx) % pw.len();

    //     lookup_table.insert(after_idx, rotation);

    //     println!(
    //         "pw: {}, orig_idx: {}, after_idx: {}",
    //         password.iter().collect::<String>(),
    //         orig_idx,
    //         after_idx
    //     );
    // }

    // dbg!(lookup_table);

    println!("part 1 => {}", password.iter().collect::<String>());

    let mut password: Vec<char> = "fbgdceah".to_string().chars().collect();
    for input_store in instructions.iter().rev() {
        input_store.unapply(&mut password);
    }
    println!("part 2 => {}", password.iter().collect::<String>());
}

fn apply_inst(chars: Vec<char>, inst: Instruction) -> Vec<char> {
    todo!()
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

    fn apply_instruction(&mut self, instruction: Instruction) {}
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
