use advent::{
    input_store,
    parsers::{parse_isize, ws},
};
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag};

#[derive(Debug, Clone)]
enum Plane {
    X(isize),
    Y(isize),
    Z(isize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Range(isize, isize);

impl Range {
    fn intersects(&self, other: &Self) -> bool {
        !(self.1 < other.0 || self.0 > other.1)
    }

    fn within(&self, other: &Self) -> bool {
        other.0 <= self.0 && self.1 <= other.1
    }

    fn len(&self) -> usize {
        (self.1 - self.0 + 1).abs() as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Region {
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

impl Region {
    fn planes(&self) -> Vec<Plane> {
        vec![
            Plane::X(self.x_range.0),
            Plane::X(self.x_range.1 + 1),
            Plane::Z(self.z_range.0),
            Plane::Z(self.z_range.1 + 1),
            Plane::Y(self.y_range.0),
            Plane::Y(self.y_range.1 + 1),
        ]
    }

    fn volume(&self) -> usize {
        self.x_range.len() * self.y_range.len() * self.z_range.len()
    }

    fn within(&self, other: &Self) -> bool {
        self.x_range.within(&other.x_range)
            && self.y_range.within(&other.y_range)
            && self.z_range.within(&other.z_range)
    }

    fn intersects(&self, other: &Plane) -> bool {
        match other {
            Plane::X(x) => self.x_range.0 < *x && *x <= self.x_range.1,
            Plane::Y(y) => self.y_range.0 < *y && *y <= self.y_range.1,
            Plane::Z(z) => self.z_range.0 < *z && *z <= self.z_range.1,
        }
    }

    fn intersections(&self, other: &Self) -> Vec<Plane> {
        self.planes()
            .iter()
            .filter(|&a| other.intersects(a))
            .cloned()
            .collect()
    }

    fn has_intersections(&self, other: &Self) -> bool {
        self.x_range.intersects(&other.x_range)
            && self.y_range.intersects(&other.y_range)
            && self.z_range.intersects(&other.z_range)
    }

    fn cut(self, at: &Plane) -> Option<(Region, Region)> {
        if !self.intersects(&at) {
            return None;
        }

        let pair = match at {
            Plane::X(x) => (
                Region {
                    x_range: Range(self.x_range.0, x - 1),
                    ..self
                },
                Region {
                    x_range: Range(*x, self.x_range.1),
                    ..self
                },
            ),
            Plane::Y(y) => (
                Region {
                    y_range: Range(self.y_range.0, y - 1),
                    ..self
                },
                Region {
                    y_range: Range(*y, self.y_range.1),
                    ..self
                },
            ),
            Plane::Z(z) => (
                Region {
                    z_range: Range(self.z_range.0, z - 1),
                    ..self
                },
                Region {
                    z_range: Range(*z, self.z_range.1),
                    ..self
                },
            ),
        };

        Some(pair)
    }

    fn separate(&self, others: Vec<Self>) -> Vec<Self> {
        let mut out = Vec::new();
        let mut staged = others;

        while !staged.is_empty() {
            let candidate = staged.pop().unwrap();
            if candidate.within(self) {
                continue;
            }
            if !self.has_intersections(&candidate) {
                out.push(candidate);
                continue;
            }

            let intersections = self.intersections(&candidate);
            let cut_plane = intersections.first().unwrap();
            let (a, b) = candidate.cut(cut_plane).unwrap();

            vec![a, b].iter().for_each(|&v| {
                if v.has_intersections(self) {
                    staged.push(v)
                } else {
                    out.push(v)
                }
            })
        }

        out
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.x_range.0,
            self.x_range.1,
            self.y_range.0,
            self.y_range.1,
            self.z_range.0,
            self.z_range.1
        )
    }
}

struct Field(Vec<Region>);

impl Field {
    fn apply(self, instruction: &Instruction) -> Self {
        let mut field = instruction.region.separate(self.0);

        if instruction.target {
            field.push(instruction.region)
        }

        Self(field)
    }

    fn new() -> Self {
        Self(Vec::new())
    }

    fn volume(&self) -> usize {
        self.0.iter().map(|r| r.volume()).sum()
    }
}

#[derive(Debug)]
struct Instruction {
    target: bool,
    region: Region,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        parse_inst(input).unwrap().1
    }
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, a) = parse_isize(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, b) = parse_isize(input)?;

    let left = [a, b].iter().min().unwrap().clone();
    let right = [a, b].iter().max().unwrap().clone();
    Ok((input, Range(left, right)))
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, _) = tag("x=")(input)?;
    let (input, x_range) = ws(parse_range)(input)?;
    let (input, _) = tag(",y=")(input)?;
    let (input, y_range) = ws(parse_range)(input)?;
    let (input, _) = tag(",z=")(input)?;
    let (input, z_range) = ws(parse_range)(input)?;

    Ok((
        input,
        Region {
            x_range,
            y_range,
            z_range,
        },
    ))
}

fn parse_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, oper) = alt((tag("on"), tag("off")))(input)?;
    let (input, region) = ws(parse_region)(input)?;

    let target = oper == "on";

    Ok((input, Instruction { target, region }))
}

fn main() {
    let input = input_store::get_input(2021, 22);

    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| parse_inst(l.trim()).unwrap().1)
        .collect();

    let field = instructions
        .iter()
        .fold(Field::new(), |field, inst| field.apply(inst));

    // doing part 2 first
    let part_2 = field.volume();

    let disable = r#"
    off x=-100000..-51,y=-100000..100000,z=-100000..100000
    off x=51..100000,y=-100000..100000,z=-100000..100000
    off x=-100000..100000,y=-100000..-51,z=-100000..100000
    off x=-100000..100000,y=51..100000,z=-100000..100000
    off x=-100000..100000,y=-100000..100000,z=-100000..-51
    off x=-100000..100000,y=-100000..100000,z=51..100000"#;

    let disable_instructions: Vec<Instruction> = disable
        .trim()
        .lines()
        .map(|l| parse_inst(l.trim()).unwrap().1)
        .collect();

    let field = disable_instructions
        .iter()
        .fold(field, |field, inst| field.apply(inst));

    let part_1 = field.volume();

    println!("part_1 => {}", part_1);
    println!("part_2 => {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    impl From<&str> for Region {
        fn from(input: &str) -> Self {
            parse_region(input).unwrap().1
        }
    }

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[rstest]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::X(5), ("x=0..4,y=0..10,z=0..10", "x=5..10,y=0..10,z=0..10"))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Y(5), ("x=0..10,y=0..4,z=0..10", "x=0..10,y=5..10,z=0..10"))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Z(5), ("x=0..10,y=0..10,z=0..4", "x=0..10,y=0..10,z=5..10"))]
    fn cut_test(#[case] given: &str, #[case] cut: Plane, #[case] expected: (&str, &str)) {
        let given: Region = given.into();
        let expected = (expected.0.into(), expected.1.into());

        assert_eq!(given.cut(&cut), Some(expected));
    }

    #[rstest]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::X(15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Y(15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Z(15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::X(-15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Y(-15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Z(-15))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::X(11))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Y(11))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Z(11))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::X(0))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Y(0))]
    #[case("x=0..10,y=0..10,z=0..10,", Plane::Z(0))]
    fn cut_misses_test(#[case] given: &str, #[case] cut: Plane) {
        let given: Region = given.into();
        assert_eq!(given.cut(&cut), None);
    }

    #[rstest]
    #[case("x=0..10,y=0..10,z=0..10,", "x=4..6,y=4..6,z=4..6,", 6)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..10,y=0..1,z=0..1,", 2)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..1,y=0..10,z=0..1,", 2)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..1,y=0..1,z=0..10,", 2)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..5,y=0..1,z=0..1,", 3)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..1,y=0..5,z=0..1,", 3)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..1,y=0..1,z=0..5,", 3)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=5..5,y=5..5,z=0..10,", 4)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=5..5,y=0..10,z=5..5,", 4)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..10,y=5..5,z=5..5,", 4)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..10,y=5..5,z=5..5,", 4)]

    fn separate_test(#[case] given: &str, #[case] region: &str, #[case] expected: usize) {
        let given: Region = given.into();
        let region: Region = region.into();

        let result = region.separate(vec![given]);
        dbg!(&result);

        assert_eq!(result.len(), expected);
        assert_eq!(Field(result).volume(), given.volume() - region.volume());
    }

    #[rstest]
    #[case("x=0..10,y=0..10,z=0..10,", "x=4..6,y=4..6,z=4..6,", true)]
    #[case("x=4..6,y=4..6,z=4..6,", "x=0..10,y=0..10,z=0..10,", false)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..6,y=0..6,z=0..6,", true)]
    #[case("x=0..10,y=0..10,z=0..10,", "x=0..6,y=4..6,z=-1..11,", false)]
    #[trace]
    fn within_test(#[case] given: &str, #[case] region: &str, #[case] expected: bool) {
        let given: Region = given.into();
        let region: Region = region.into();

        assert_eq!(region.within(&given), expected)
    }

    impl From<&str> for Range {
        fn from(input: &str) -> Self {
            parse_range(input).unwrap().1
        }
    }

    #[rstest]
    #[case(("0..10", "4..6"), true)]
    #[case(("0..10", "5..15"), true)]
    #[case(("0..10", "10..15"), true)]
    #[case(("0..10", "11..15"), false)]
    #[case(("0..10", "20..30"), false)]
    #[trace]
    fn range_intersection_test(#[case] given: (&str, &str), #[case] expected: bool) {
        let a: Range = given.0.into();
        let b: Range = given.1.into();

        dbg!(&a, &b);

        let result = a.intersects(&b);
        assert_eq!(result, b.intersects(&a));
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("on x=0..0,y=0..0,z=0..0", 1)]
    #[case("on x=0..0,y=0..0,z=0..1", 2)]
    #[case("on x=0..1,y=0..1,z=0..1", 8)]
    #[case("on x=0..2,y=0..2,z=0..2", 27)]
    #[case("on x=-1..2,y=-1..2,z=-1..2", 64)]
    fn p1_tests(#[case] given: &str, #[case] expected: usize) {
        let instruction: Instruction = given.into();

        let field = Field::new();
        let result = field.apply(&instruction);

        assert_eq!(result.volume(), expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
