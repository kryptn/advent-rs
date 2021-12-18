use advent::{
    grid::{coordinates_within, Coordinate},
    input_store,
    parsers::parse_isize,
};
use nom::{
    bytes::complete::{tag, take},
    IResult,
};

struct Probe<'a> {
    position: Coordinate,
    velocity: Coordinate,
    target: Option<&'a (Coordinate, Coordinate)>,
    //highest_y: i32,
}

impl<'a> Probe<'a> {
    fn new(velocity: Coordinate, target: Option<&'a (Coordinate, Coordinate)>) -> Self {
        Self {
            position: (0, 0).into(),
            velocity,
            //highest_y: 0,
            target,
        }
    }

    fn step(&self) -> Self {
        let position = self.position + self.velocity;
        let velocity = {
            let x = if self.velocity.x > 0 {
                self.velocity.x - 1
            } else if self.velocity.x < 0 {
                self.velocity.x + 1
            } else {
                self.velocity.x
            };

            let y = self.velocity.y - 1;
            (x, y).into()
        };

        Self {
            position,
            velocity,
            target: self.target,
        }
    }

    fn out_of_bounds(&self) -> bool {
        self.position.y <= -200 || self.position.x <= -40 || self.position.x >= 100
    }

    fn in_range(&self, a: Coordinate, b: Coordinate) -> bool {
        a.x <= self.position.x
            && self.position.x <= b.x
            && a.y <= self.position.y
            && self.position.y <= b.y
    }
}

fn in_range(pos: Coordinate, a: Coordinate, b: Coordinate) -> bool {
    a.x <= pos.x && pos.x <= b.x && a.y <= pos.y && pos.y <= b.y
}

impl<'a> Iterator for Probe<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(target) = self.target {
            if self.in_range(target.0, target.1) {
                return None;
            }
        }

        if self.out_of_bounds() {
            return None;
        }

        *self = self.step();
        Some(self.position)
    }
}

fn scan_range(target_area: &(Coordinate, Coordinate)) -> Vec<(Coordinate, i32)> {
    let mut valid = Vec::new();

    for velocity in coordinates_within((0, -200).into(), (100, 10000).into()) {
        let probe = Probe::new(velocity, Some(target_area));

        let mut max_y = 0;
        let mut positions = Vec::new();
        for pos in probe {
            if pos.y > max_y {
                max_y = pos.y
            }
            positions.push(pos);
        }

        if in_range(
            positions.iter().last().unwrap().clone(),
            target_area.0,
            target_area.1,
        ) {
            valid.push((velocity, max_y));
        }
    }

    valid
}

fn parse_range(input: &str) -> IResult<&str, [isize; 2]> {
    let (input, _) = take(2usize)(input)?;
    let (input, a) = parse_isize(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, b) = parse_isize(input)?;

    Ok((input, [a, b]))
}

fn parse_area(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    let (input, _) = tag("target area: ")(input)?;
    let (input, x) = parse_range(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, y) = parse_range(input)?;

    let x1 = x.iter().min().unwrap().clone();
    let x2 = x.iter().max().unwrap().clone();

    let y1 = y.iter().min().unwrap().clone();
    let y2 = y.iter().max().unwrap().clone();

    return Ok((
        input,
        ((x1 as i32, y1 as i32).into(), (x2 as i32, y2 as i32).into()),
    ));
}

fn main() {
    let input = input_store::get_input(2021, 17);
    let input = input.trim();

    let target_area = parse_area(input).unwrap().1;

    let valid_velocities = scan_range(&target_area);

    let max_y = valid_velocities.iter().map(|(_, y)| y).max().unwrap();

    println!("part_1 => {}", max_y);
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
