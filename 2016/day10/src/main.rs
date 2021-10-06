use std::collections::HashMap;

use advent::{
    input_store,
    parsers::{parse_usize, ws},
};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Destination {
    Output(usize),
    Bot(usize),
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct Cmp {
    bot: usize,
    high: Destination,
    low: Destination,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct GiveValue {
    value: usize,
    to: Destination,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Instruction {
    Give(GiveValue),
    Compare(Cmp),
}

impl<'a> From<&'a str> for Instruction {
    fn from(input: &'a str) -> Self {
        let (_, inst) = parse_instruction(input).unwrap();
        inst
    }
}

fn parse_give_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("value"))(input)?;
    let (input, value) = parse_usize(input)?;
    let (input, _) = tuple((ws(tag("goes")), ws(tag("to")), ws(tag("bot"))))(input)?;
    let (input, bot) = parse_usize(input)?;
    let to = Destination::Bot(bot);
    Ok((input, Instruction::Give(GiveValue { value, to })))
}

fn parse_destination(input: &str) -> IResult<&str, Destination> {
    let (input, dest) = alt((ws(tag("bot")), ws(tag("output"))))(input)?;
    let (input, to) = parse_usize(input)?;

    let dest = match dest {
        "bot" => Destination::Bot(to),
        "output" => Destination::Output(to),
        _ => unreachable!(),
    };

    Ok((input, dest))
}

fn parse_cmp_inst(input: &str) -> IResult<&str, Instruction> {
    // bot 103 gives low to [bot|output] 13 and high to bot 125
    let (input, _) = ws(tag("bot"))(input)?;
    let (input, bot) = parse_usize(input)?;
    let (input, _) = tuple((ws(tag("gives")), ws(tag("low")), ws(tag("to"))))(input)?;
    let (input, low) = parse_destination(input)?;
    let (input, _) = tuple((ws(tag("and")), ws(tag("high")), ws(tag("to"))))(input)?;
    let (input, high) = parse_destination(input)?;

    Ok((input, Instruction::Compare(Cmp { bot, high, low })))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_cmp_inst, parse_give_inst))(input)
}

fn append<T>(m: &mut HashMap<usize, Vec<T>>, key: usize, value: T) {
    if !m.contains_key(&key) {
        let v = vec![value];
        m.insert(key, v);
    } else {
        m.get_mut(&key).unwrap().push(value);
    }
}

fn solve(input: &str, look_for: (usize, usize)) -> (usize, usize) {
    let mut instructions: Vec<Instruction> = input.lines().map(|line| line.into()).collect();
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut compares: HashMap<(usize, usize), usize> = HashMap::new();

    while !instructions.is_empty() {
        for i in 0..instructions.len() {
            let inst = instructions[i].clone();

            match inst {
                Instruction::Give(gv) => {
                    instructions.remove(i);
                    match gv.to {
                        Destination::Output(out) => append(&mut outputs, out, gv.value),
                        Destination::Bot(bot) => append(&mut bots, bot, gv.value),
                    }
                    break;
                }
                Instruction::Compare(cmp) => {
                    if let Some(chips) = bots.get(&cmp.bot) {
                        if chips.len() == 2 {
                            let a = chips[0];
                            let b = chips[1];

                            let (low, high) = if a > b { (b, a) } else { (a, b) };

                            compares.insert((low, high), cmp.bot);

                            match cmp.low {
                                Destination::Output(out) => append(&mut outputs, out, low),
                                Destination::Bot(bot) => append(&mut bots, bot, low),
                            }

                            match cmp.high {
                                Destination::Output(out) => append(&mut outputs, out, high),
                                Destination::Bot(bot) => append(&mut bots, bot, high),
                            }

                            instructions.remove(i);
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }

    let cmpd = compares.get(&look_for).unwrap().to_owned();
    let muld = outputs[&0][0] * outputs[&1][0] * outputs[&2][0];

    (cmpd, muld)
}

fn main() {
    let input = input_store::get_input(2016, 10);
    let (p1, p2) = solve(&input, (17, 61));

    println!("part 1 => {:?}", p1);
    println!("part 2 => {:?}", p2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn parse() {
        let inst: Instruction = "value 5 goes to bot 2".into();
        let expected = Instruction::Give(GiveValue {
            value: 5,
            to: Destination::Bot(2),
        });
        assert_eq!(inst, expected);

        let inst: Instruction = "bot 2 gives low to bot 1 and high to bot 0".into();
        let expected = Instruction::Compare(Cmp {
            bot: 2,
            low: Destination::Bot(1),
            high: Destination::Bot(0),
        });
        assert_eq!(inst, expected);

        let inst: Instruction = "value 3 goes to bot 1".into();
        let expected = Instruction::Give(GiveValue {
            value: 3,
            to: Destination::Bot(1),
        });
        assert_eq!(inst, expected);

        let inst: Instruction = "bot 1 gives low to output 1 and high to bot 0".into();
        let expected = Instruction::Compare(Cmp {
            bot: 1,
            low: Destination::Output(1),
            high: Destination::Bot(0),
        });
        assert_eq!(inst, expected);

        let inst: Instruction = "bot 0 gives low to output 2 and high to output 0".into();
        let expected = Instruction::Compare(Cmp {
            bot: 0,
            low: Destination::Output(2),
            high: Destination::Output(0),
        });
        assert_eq!(inst, expected);

        let inst: Instruction = "value 2 goes to bot 2".into();
        let expected = Instruction::Give(GiveValue {
            value: 2,
            to: Destination::Bot(2),
        });
        assert_eq!(inst, expected);
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
