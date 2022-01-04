use std::collections::HashMap;

use advent::{
    grid::Grid,
    input_store,
    parsers::{parse_isize, ws},
};
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag, character::complete::one_of};

#[derive(Debug)]
struct Instruction {
    target: bool,
    region: Region,
}

fn parse_range(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = one_of("xyz")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, a) = parse_isize(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, b) = parse_isize(input)?;

    let left = [a, b].iter().min().unwrap().clone();
    let right = [a, b].iter().max().unwrap().clone();
    Ok((input, (left, right)))
}

fn parse_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, oper) = alt((tag("on"), tag("off")))(input)?;
    let (input, x) = ws(parse_range)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = ws(parse_range)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = ws(parse_range)(input)?;

    Ok((
        input,
        Instruction {
            target: oper == "on",
            region: Region {
                x_range: x,
                y_range: y,
                z_range: z,
            },
        },
    ))
}

fn main() {
    let input = input_store::get_input(2021, 22);

    let input = r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#;

    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| parse_inst(l.trim()).unwrap().1)
        .collect();

    // dbg!(&instructions);

    let mut grid = HashMap::new();

    for instruction in instructions {
        dbg!(&instruction);
        for x in instruction.region.x_range.0..=instruction.region.x_range.1 {
            for y in instruction.region.y_range.0..=instruction.region.y_range.1 {
                for z in instruction.region.z_range.0..=instruction.region.z_range.1 {
                    let c = [x, y, z];
                    let g = grid.entry(c).or_insert(false);
                    *g = instruction.target;
                }
            }
        }
    }

    let count = grid.iter().filter(|(_, v)| **v).count();

    println!("part_1 => {}", count);
    println!("part_2 => {}", "not done");
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
