use std::collections::HashSet;

use advent::{grid, input_store};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Clone)]
struct Direction {
    dir: grid::RelativeDirection,
    steps: i32,
}

fn parse_cardinal(input: &str) -> IResult<&str, grid::RelativeDirection> {
    let (input, dir) = one_of("UDLR")(input)?;
    Ok((input, dir.into()))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = parse_cardinal(input)?;
    let (input, steps) = digit1(input)?;
    let steps: i32 = steps.parse().unwrap();

    Ok((input, Direction { dir, steps }))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list0(tag(", "), parse_direction)(input)
}

fn main() {
    let input = input_store::get_input(2016, 1);
    //let input = "R2, L3";
    //let input = "R8, R4, R4, R8";
    let (_, directions) = parse_directions(&input).unwrap();
    dbg!(&directions);

    let mut position = grid::Coordinate::new(0, 0);
    let mut heading = grid::Coordinate::new(0, 1);
    for dir in directions.clone() {
        heading = heading.turn(dir.dir);
        position = position + heading.scale(dir.steps);
    }

    let zero = grid::Coordinate::new(0, 0);
    println!("part 1 => {}", grid::manhattan(zero, position));

    let directions: Vec<Direction> = directions
        .iter()
        .map(|d| {
            let mut out = vec![Direction {
                dir: d.dir,
                steps: 1,
            }];
            for _ in 0..d.steps - 1 {
                out.push(Direction {
                    dir: grid::RelativeDirection::Up,
                    steps: 1,
                });
            }
            out
        })
        .flatten()
        .collect();

    let mut position = grid::Coordinate::new(0, 0);
    let mut heading = grid::Coordinate::new(0, 1);

    let mut visited: HashSet<grid::Coordinate> = HashSet::new();
    visited.insert(position);

    for dir in directions {
        heading = heading.turn(dir.dir);
        position = position + heading.scale(dir.steps);

        if visited.contains(&position) {
            println!("visited {:?} before", position);
            break;
        }
        visited.insert(position);
    }

    println!("part 2 => {}", grid::manhattan(zero, position));
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
