use advent::{
    grid::{manhattan, Coordinate},
    input_store,
    parsers::parse_isize,
};
use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::opt, IResult};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = parse_isize(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = parse_isize(input)?;
    let (input, _) = opt(tag(": "))(input)?;
    Ok((input, (x, y).into()))
}

fn parse_detection(input: &str) -> IResult<&str, Detection> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_coordinate(input)?;
    let (input, _) = tag("closest beacon is at ")(input)?;
    let (input, beacon) = parse_coordinate(input)?;

    Ok((input, Detection { sensor, beacon }))
}

#[derive(Debug)]
struct Detection {
    sensor: Coordinate,
    beacon: Coordinate,
}

impl Detection {
    fn distance(&self) -> i64 {
        manhattan(self.sensor, self.beacon)
    }

    fn range_at(&self, y: i64) -> Option<(i64, i64)> {
        let delta = self.distance() - (self.sensor.y - y).abs();
        if delta < 0 {
            None
        } else {
            Some((self.sensor.x - delta, self.sensor.x + delta))
        }
    }
}

fn detection_ranges_at(detections: &Vec<Detection>, y: i64) -> Vec<(i64, i64)> {
    let ranges: Vec<_> = detections.iter().filter_map(|d| d.range_at(y)).collect();
    join_ranges(ranges)
}

fn join_ranges(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
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
    let max_x = 4000000;

    let detections: Vec<Detection> = input
        .trim()
        .lines()
        .map(|line| {
            let (_, detection) = parse_detection(line.trim()).unwrap();
            detection
        })
        .collect();

    let joined = detection_ranges_at(&detections, max_x / 2);
    let (a, b) = joined.first().unwrap();
    println!("part_1 => {}", (b - a).abs());

    let part_2: Coordinate = (0..=max_x)
        .into_par_iter()
        .filter_map(|y| {
            let joined_ranges = detection_ranges_at(&detections, y);
            if joined_ranges.len() > 1 {
                let x = joined_ranges.first().unwrap().1 + 1;
                Some(Coordinate::from((x, y)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .clone();

    println!(
        "part_2 => {}",
        (part_2.x as u64 * 4000000) + part_2.y as u64
    );
}

#[cfg(test)]
mod test {

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
