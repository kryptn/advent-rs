use std::ops::Add;

use advent::input_store;
use itertools::Itertools;

#[derive(Default, Clone, Debug)]
struct Coordinate {
    q: isize,
    s: isize,
    r: isize,
}

impl Coordinate {
    fn distance(&self) -> isize {
        self.q.abs() + self.s.abs() + self.r.abs()
    }

    fn towards_origin(&self) -> Direction {
        let value: Vec<Direction> = Direction::all()
            .iter()
            .cloned()
            .sorted_by(|a, b| {
                let a = (a.vector() + self).distance();
                let b = (b.vector() + self).distance();
                a.cmp(&b)
            })
            .collect();

        value[0]
    }

    fn steps_from_origin(&self) -> usize {
        let mut position = self.clone();
        let mut steps = 0;
        while position.distance() > 0 {
            position = position.step(position.towards_origin());
            steps += 1
        }
        steps
    }
}

impl From<(isize, isize, isize)> for Coordinate {
    fn from((q, s, r): (isize, isize, isize)) -> Self {
        Self { q, s, r }
    }
}

impl Add<&Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: &Coordinate) -> Self::Output {
        (self.q + rhs.q, self.s + rhs.s, self.r + rhs.r).into()
    }
}

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "nw" => Direction::NW,
            "s" => Direction::S,
            "se" => Direction::SE,
            "sw" => Direction::SW,
            _ => panic!(),
        }
    }
}

impl Direction {
    // https://www.redblobgames.com/grids/hexagons/
    fn vector(&self) -> Coordinate {
        match self {
            Direction::N => (0, 1, -1),
            Direction::NE => (1, 0, -1),
            Direction::NW => (-1, 1, 0),
            Direction::S => (0, -1, 1),
            Direction::SE => (1, -1, 0),
            Direction::SW => (-1, 0, 1),
        }
        .into()
    }

    fn all() -> [Direction; 6] {
        [
            Direction::N,
            Direction::NE,
            Direction::NW,
            Direction::S,
            Direction::SE,
            Direction::SW,
        ]
    }
}

impl Coordinate {
    fn step(&self, dir: Direction) -> Self {
        dir.vector() + self
    }
}

fn main() {
    let input: Vec<Direction> = input_store::get_input(2017, 11)
        .trim()
        .split(",")
        .map(|dir| dir.into())
        .collect();

    let mut position = Coordinate::default();
    let mut distances_from_origin = Vec::new();
    for direction in input {
        position = position + &direction.vector();
        distances_from_origin.push(position.steps_from_origin());
    }

    println!("part_1 => {}", position.steps_from_origin());
    println!("part_2 => {}", distances_from_origin.iter().max().unwrap());
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
