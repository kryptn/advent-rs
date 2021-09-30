use advent::input_store;
use itertools::Itertools;
use nom::{combinator::opt, multi, IResult};

use nom::character::complete as ch;

fn parse_digit(input: &str) -> IResult<&str, i32> {
    let (input, _) = ch::space0(input)?;

    let (input, dgt) = ch::digit1(input)?;
    Ok((input, dgt.parse::<i32>().unwrap()))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, (out, _)) = multi::many_till(parse_digit, ch::newline)(input)?;
    let (input, _) = opt(ch::newline)(input)?;
    Ok((input, out))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    multi::many0(parse_line)(input)
}

fn valid_triangle(sides: Vec<i32>) -> bool {
    let sides: Vec<i32> = sides.iter().cloned().sorted().collect();

    sides[0] + sides[1] > sides[2]
}

fn main() {
    let input = input_store::get_input(2016, 3);
    //     let input = r#"  566  477  376
    //   575  488  365
    //    50   18  156
    // "#;
    let (_, triangles) = parse_lines(&input).unwrap();

    //dbg!(&triangles);

    let valids: Vec<Vec<i32>> = triangles
        .iter()
        .cloned()
        .filter(|t| valid_triangle(t.clone()))
        .collect();

    println!(
        "part 1 => {}, triangles.len = {}",
        valids.len(),
        triangles.len()
    );

    let mut fixed: Vec<Vec<i32>> = Vec::new();

    for chunk in &triangles.into_iter().chunks(3) {
        let chunk: Vec<Vec<i32>> = chunk.collect();
        let a = vec![chunk[0][0], chunk[1][0], chunk[2][0]];
        let b = vec![chunk[0][1], chunk[1][1], chunk[2][1]];
        let c = vec![chunk[0][2], chunk[1][2], chunk[2][2]];

        fixed.extend(vec![a, b, c]);
    }

    let valids: Vec<Vec<i32>> = fixed
        .iter()
        .cloned()
        .filter(|t| valid_triangle(t.clone()))
        .collect();

    println!("part 2 => {}, fixed len {}", valids.len(), fixed.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
