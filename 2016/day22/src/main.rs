use advent::{
    input_store,
    parsers::{parse_num, ws},
    space::Coordinate2d,
};
use itertools::{self, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
//#[parse(parse_node)]
struct Node {
    size: u32,
    used: u32,
    avail: u32,
    used_pct: u32,
    coord: Coordinate2d,
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let (_, node) = parse_node(input).unwrap();
        node
    }
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate2d> {
    let (input, _) = char('x')(input)?;
    let (input, x) = parse_num::<i32>(input)?;
    let (input, _) = char('-')(input)?;
    let (input, _) = char('y')(input)?;
    let (input, y) = parse_num::<i32>(input)?;

    return Ok((input, Coordinate2d::new(x, y)));
}

fn parse_stat(input: &str) -> IResult<&str, u32> {
    let (input, n) = ws(parse_num)(input)?;
    let (input, _) = one_of("T%")(input)?;

    Ok((input, n))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("/dev/grid/node-")(input)?;
    let (input, coord) = parse_coordinate(input)?;
    let (input, (size, used, avail, used_pct)) =
        tuple((parse_stat, parse_stat, parse_stat, parse_stat))(input)?;

    Ok((
        input,
        Node {
            coord,
            size,
            used,
            avail,
            used_pct,
        },
    ))
}

impl Node {
    fn viable(&self, other: &Self) -> bool {
        return self.used > 0 && self.coord != other.coord && self.used <= other.avail;
    }
}

fn part_1(machines: Vec<Node>) -> impl std::fmt::Display {
    machines
        .iter()
        .permutations(2)
        .filter(|x| {
            let a = x.get(0).unwrap();
            let b = x.get(1).unwrap();
            a.viable(b)
        })
        .count()
}

fn main() {
    let input = input_store::get_input(2016, 22);

    let machines: Vec<Node> = input.lines().skip(2).map(|l| l.into()).collect();

    //dbg!(&machines);

    println!("part 1 => {}", part_1(machines));
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
