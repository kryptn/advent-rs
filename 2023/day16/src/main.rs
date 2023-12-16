use std::{
    collections::{HashSet, VecDeque},
    default,
};

use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Space};
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Tile {
    LeftMirror,
    RightMirror,
    VerticalSplitter,
    HorizontalSplitter,
    #[default]
    Empty,
}

impl From<char> for Tile {
    fn from(s: char) -> Self {
        match s {
            '/' => Tile::LeftMirror,
            '\\' => Tile::RightMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            '.' => Tile::Empty,
            _ => panic!("invalid tile: {}", s),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::LeftMirror => '/',
            Tile::RightMirror => '\\',
            Tile::VerticalSplitter => '|',
            Tile::HorizontalSplitter => '-',
            Tile::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct TilePower {
    power: usize,
}

impl std::fmt::Display for TilePower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.power {
            0 => '.',
            _ => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Light {
    position: Coordinate,
    direction: Coordinate,
    steps: usize,
}

impl Light {
    fn new(position: Coordinate, direction: Coordinate) -> Self {
        Self {
            position,
            direction,
            steps: 0,
        }
    }

    fn with(&self, position: Coordinate, direction: Coordinate) -> Self {
        Self {
            position,
            direction,
            steps: self.steps + 1,
        }
    }

    fn step(&self, grid: &Space<Coordinate, Tile>) -> (Option<Self>, Option<Self>) {
        let next = self.position + self.direction;
        let next_tile = if let Some(t) = grid.get(&next) {
            t
        } else {
            return (None, None);
        };

        // println!("at: {:?} => {:?}", self, next_tile);
        match next_tile {
            Tile::LeftMirror => match self.direction {
                spatial::RIGHT => (Some(self.with(next, spatial::DOWN)), None),
                spatial::LEFT => (Some(self.with(next, spatial::UP)), None),
                spatial::UP => (Some(self.with(next, spatial::LEFT)), None),
                spatial::DOWN => (Some(self.with(next, spatial::RIGHT)), None),
                _ => panic!("invalid direction: {:?}", self.direction),
            },
            Tile::RightMirror => match self.direction {
                spatial::RIGHT => (Some(self.with(next, spatial::UP)), None),
                spatial::LEFT => (Some(self.with(next, spatial::DOWN)), None),
                spatial::UP => (Some(self.with(next, spatial::RIGHT)), None),
                spatial::DOWN => (Some(self.with(next, spatial::LEFT)), None),
                _ => panic!("invalid direction: {:?}", self.direction),
            },
            Tile::VerticalSplitter => match self.direction {
                spatial::LEFT | spatial::RIGHT => (
                    Some(self.with(next, spatial::UP)),
                    Some(self.with(next, spatial::DOWN)),
                ),
                _ => (Some(self.with(next, self.direction)), None),
            },
            Tile::HorizontalSplitter => match self.direction {
                spatial::UP | spatial::DOWN => (
                    Some(self.with(next, spatial::LEFT)),
                    Some(self.with(next, spatial::RIGHT)),
                ),
                _ => (Some(self.with(next, self.direction)), None),
            },
            Tile::Empty => (Some(self.with(next, self.direction)), None),
        }
    }
}

fn shine_light(grid: &Space<Coordinate, Tile>, light: Light) -> HashSet<(Coordinate, Coordinate)> {
    let mut light_beam_queue: VecDeque<_> = vec![light].into_iter().collect();
    let mut energized = HashSet::new();

    while let Some(light) = light_beam_queue.pop_front() {
        let (next, split_light) = light.step(grid);

        match next {
            Some(next) => {
                let key = (next.position, next.direction);
                if !energized.contains(&key) {
                    energized.insert(key);
                    light_beam_queue.push_back(next);
                }
            }
            _ => {}
        }

        match split_light {
            Some(split) => {
                let key = (split.position, split.direction);
                if !energized.contains(&key) {
                    energized.insert(key);
                    light_beam_queue.push_back(split);
                }
            }
            _ => {}
        }
    }
    energized
}

fn total_energized(cave: &Space<Coordinate, Tile>, light: Light) -> usize {
    let part_1 = shine_light(cave, light);

    let energized_space: Space<Coordinate, TilePower> = part_1
        .iter()
        .map(|(c, _)| c)
        .unique()
        .map(|c| (*c, TilePower { power: 1 }))
        .collect();

    energized_space.len()
}

fn starting_positions(cave: &Space<Coordinate, Tile>) -> Vec<Light> {
    let (lower, upper) = cave.bounding_box();
    let mut out = vec![];

    for x in lower.x..=upper.x {
        out.push(Light::new((x, lower.y - 1).into(), spatial::UP));
        out.push(Light::new((x, upper.y + 1).into(), spatial::DOWN));
    }

    for y in lower.y..=upper.y {
        out.push(Light::new((lower.x - 1, y).into(), spatial::RIGHT));
        out.push(Light::new((upper.x + 1, y).into(), spatial::LEFT));
    }

    out
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#".|...\....
    // |.-.\.....
    // .....|-...
    // ........|.
    // ..........
    // .........\
    // ..../.\\..
    // .-.-/..|..
    // .|....-|.\
    // ..//.|...."#;

    let cave: Space<Coordinate, Tile> = Space::from(input);

    // println!("{}", cave);

    let p1_start = Light::new((-1, 0).into(), spatial::RIGHT);
    let p1 = total_energized(&cave, p1_start);
    println!("part_1 => {}", p1);

    let p2 = starting_positions(&cave)
        .iter()
        .map(|l| {
            let t = total_energized(&cave, *l);
            // dbg!(l, t);
            t
        })
        .max()
        .unwrap();

    println!("part_2 => {}", p2);
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
