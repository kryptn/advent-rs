use advent::{
    input_store,
    parsers::{parse_num, ws},
};
use advent_toolbox::spatial::{Coordinate, Space};
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
    coord: Coordinate,
    goal: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            size: 0,
            used: 0,
            avail: 0,
            used_pct: 0,
            coord: Coordinate::new(0, 0),
            goal: false,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = if self.goal {
            "G"
        } else if self.used > 100 {
            "#"
        } else if self.used > 10 {
            "."
        } else {
            "_"
        };
        write!(f, " {} ", repr)
    }
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let (_, node) = parse_node(input).unwrap();
        node
    }
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = char('x')(input)?;
    let (input, x) = parse_num::<isize>(input)?;
    let (input, _) = char('-')(input)?;
    let (input, _) = char('y')(input)?;
    let (input, y) = parse_num::<isize>(input)?;

    return Ok((input, Coordinate::new(x, y)));
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
            goal: false,
        },
    ))
}

impl Node {
    fn viable(&self, other: &Self) -> bool {
        return self.used > 0 && self.coord != other.coord && self.used <= other.avail;
    }
}

fn part_1(machines: &Vec<Node>) -> impl std::fmt::Display {
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

fn part_2(machines: &Space<Coordinate, Node>) -> usize {
    let goal = machines
        .iter()
        .find(|(_, n)| n.goal)
        .map(|(c, _)| c)
        .unwrap()
        .clone();

    let empty = machines
        .iter()
        .find(|(_, n)| n.used == 0)
        .map(|(c, _)| c)
        .unwrap()
        .clone();
    let origin = (0, 0).into();
    let initial = empty.distance(&origin);
    let to_goal = origin.distance(&goal) - 1;
    let loops = (goal.x as usize - 1) * 5;
    let total = initial + to_goal + loops + 1;

    // println!("origin: {}, empty: {}, goal: {}", origin, empty, goal);
    // println!("empty to origin: {}", initial);
    // println!("origin to to_goal: {}", to_goal);
    // println!("loops: {}, loop total: {}", loops/5, loops);
    // println!("total: {}", total);
    return total;
}

fn main() {
    let input = input_store::get_input(2016, 22);
    // this example won't work because my input had a solid line of full servers,
    // forcing the most optimal solution to traverse to the origin.
    //     let input = r#"Filesystem            Size  Used  Avail  Use%
    //
    // /dev/grid/node-x0-y0   10T    8T     2T   80%
    // /dev/grid/node-x0-y1   11T    6T     5T   54%
    // /dev/grid/node-x0-y2   32T   28T     4T   87%
    // /dev/grid/node-x1-y0    9T    7T     2T   77%
    // /dev/grid/node-x1-y1    8T    0T     8T    0%
    // /dev/grid/node-x1-y2   11T    7T     4T   63%
    // /dev/grid/node-x2-y0   10T    6T     4T   60%
    // /dev/grid/node-x2-y1    9T    8T     1T   88%
    // /dev/grid/node-x2-y2    9T    6T     3T   66%"#;

    let machines: Vec<Node> = input.lines().skip(2).map(|l| l.into()).collect();
    println!("part 1 => {}", part_1(&machines));

    let mhm: Vec<_> = machines.clone().into_iter().map(|m| (m.coord, m)).collect();
    let mut space: Space<_, _> = mhm.into();
    let (lower, upper) = space.bounding_box();
    space
        .entry((upper.x, 0).into())
        .and_modify(|e| e.goal = true);

    println!("part 2 => {}", part_2(&space));

    println!("{}", space);
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
