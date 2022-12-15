use advent::{
    grid::{manhattan, Coordinate},
    input_store,
    parsers::parse_isize,
};
use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::opt, IResult};

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = parse_isize(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = parse_isize(input)?;
    let (input, _) = opt(tag(": "))(input)?;
    Ok((input, (x, y).into()))
}

#[derive(Debug)]
struct Detection {
    sensor: Coordinate,
    beacon: Coordinate,
}

impl Detection {
    fn distance(&self) -> usize {
        manhattan(self.sensor, self.beacon) as usize
    }

    fn range_at(&self, y: i32) -> Option<(i32, i32)> {
        let delta = self.distance() as i32 - (self.sensor.y - y).abs();
        if delta < 0 {
            None
        } else {
            Some((self.sensor.x - delta, self.sensor.x + delta))
        }
    }
}

fn parse_detection(input: &str) -> IResult<&str, Detection> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_coordinate(input)?;
    let (input, _) = tag("closest beacon is at ")(input)?;
    let (input, beacon) = parse_coordinate(input)?;

    Ok((input, Detection { sensor, beacon }))
}

fn part_2(detections: &Vec<Detection>, max: i32) -> Vec<Coordinate> {
    let mut out = Vec::new();
    for y in 0..=max {
        let ranges: Vec<_> = detections.iter().filter_map(|d| d.range_at(y)).collect();
        let joined = join_ranges(ranges);
        if joined.len() > 1 {
            let x = joined.first().unwrap().1 + 1;
            out.push((x, y).into())
        }
    }
    out
}

fn join_ranges(ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let ranges: Vec<_> = ranges
        .iter()
        .cloned()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect();
    let mut out = Vec::new();

    for (a, b) in ranges {
        if out.is_empty() {
            out.push((a, b));
            continue;
        }
        let (pa, pb) = out.pop().unwrap();

        if pb + 1 >= a {
            out.push((pa, if b > pb { b } else { pb }));
        } else {
            out.push((pa, pb));
            out.push((a, b));
        }
    }
    out
}

fn main() {
    let input = input_store::get_input(2022, 15);

    let detections: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (_, detection) = parse_detection(line.trim()).unwrap();
            detection
        })
        .collect();

    let ranges: Vec<_> = detections
        .iter()
        .filter_map(|d| d.range_at(2000000))
        .collect();
    let joined = join_ranges(ranges);
    let (a, b) = joined.first().unwrap();

    println!("part_1 => {}", (b - a).abs());

    let p2 = part_2(&detections, 4000000);

    let p2 = p2.first().unwrap();
    println!("part_2 => {}", (p2.x as u64 * 4000000) + p2.y as u64);
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
