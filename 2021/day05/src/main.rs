use advent::{
    grid::{line_between, Coordinate, Grid},
    input_store,
    parsers::{parse_coordinate, ws},
};
use nom::{bytes::complete::tag, IResult};

fn parse_vent(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    let (input, start) = parse_coordinate(input)?;
    let (input, _) = ws(tag("->"))(input)?;
    let (input, end) = parse_coordinate(input)?;

    Ok((input, (start, end)))
}

#[derive(Debug)]
struct Vent {
    start: Coordinate,
    end: Coordinate,
    coordinates: Vec<Coordinate>,
}

impl From<&str> for Vent {
    fn from(input: &str) -> Self {
        let (_, (start, end)) = parse_vent(input).unwrap();
        let coordinates = line_between(start, end);

        Self {
            start,
            end,
            coordinates,
        }
    }
}

fn main() {
    let input = input_store::get_input(2021, 05);

    let vents: Vec<Vent> = input.trim().lines().map(|l| l.trim().into()).collect();

    // dbg!(&vents);

    let mut floor: Grid<i32> = Grid::new();

    for vent in vents.iter() {
        if vent.start.x == vent.end.x || vent.start.y == vent.end.y {
            vent.coordinates.iter().for_each(|c| {
                if !floor.contains_key(c) {
                    floor.insert(c.clone(), 0);
                }

                *floor.get_mut(c).unwrap() += 1;
            })
        }
    }

    let intersecting = floor.iter().filter(|(_, &b)| b > 1).count();

    println!("part_1 => {}", intersecting);

    let mut floor: Grid<i32> = Grid::new();

    for vent in vents {
        vent.coordinates.iter().for_each(|c| {
            if !floor.contains_key(c) {
                floor.insert(c.clone(), 0);
            }

            *floor.get_mut(c).unwrap() += 1;
        })
    }

    let intersecting = floor.iter().filter(|(_, &b)| b > 1).count();

    println!("part_2 => {}", intersecting);
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
