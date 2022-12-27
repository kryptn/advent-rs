use advent::{input_store, parsers::parse_usize};
use advent_toolbox::spatial::{Coordinate, Space};
use itertools::Itertools;
use nom::{branch::alt, character::complete::one_of, multi::many1, sequence::tuple, IResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    None,
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        match input {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            ' ' => Tile::None,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => "#",
            Tile::Open => ".",
            Tile::None => " ",
        };
        write!(f, "{c}")
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
enum Turn {
    Left,
    Right,
}

impl From<char> for Turn {
    fn from(ch: char) -> Self {
        match ch {
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Turn::Left => "L",
            Turn::Right => "R",
        };
        write!(f, "{c}")
    }
}

#[derive(Clone)]
enum Movement {
    Forward { by: usize },
    Turn { to_the: Turn },
}

struct Directions(Vec<Movement>);

impl std::fmt::Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: Vec<String> = self.0.iter().map(|m| format!("{m}")).collect();
        write!(f, "{}", values.join(""))
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match &self {
            Movement::Forward { by } => format!("{by}"),
            Movement::Turn { to_the } => format!("{to_the}"),
        };
        write!(f, "{val}")
    }
}

fn parse_turn(input: &str) -> IResult<&str, Movement> {
    let (input, to_the) = one_of("LR")(input)?;
    let to_the = to_the.into();
    Ok((input, Movement::Turn { to_the }))
}

fn parse_forward(input: &str) -> IResult<&str, Movement> {
    let (input, by) = parse_usize(input)?;
    Ok((input, Movement::Forward { by }))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Movement>> {
    many1(alt((parse_turn, parse_forward)))(input)
}

type Grove = Space<Coordinate, Tile>;

const DIRECTIONS: [Coordinate; 4] = [
    Coordinate { x: 1, y: 0 },  // right
    Coordinate { x: 0, y: 1 },  // down
    Coordinate { x: -1, y: 0 }, // left
    Coordinate { x: 0, y: -1 }, // up
];

struct Walk {
    grove: Grove,
    directions: Vec<Movement>,

    directions_idx: usize,
    position: Coordinate,
    forward_idx: usize,
}

impl Walk {
    fn new(grove: Grove, directions: Vec<Movement>) -> Self {
        let directions_idx = 0;
        let position = grove
            .keys()
            .filter(|c| c.y == 0)
            .sorted()
            .next()
            .unwrap()
            .clone();

        let forward_idx = 0;

        Self {
            grove,
            directions,
            directions_idx,
            position,
            forward_idx,
        }
    }

    fn wrapped(&self) -> Coordinate {
        let slice: Vec<_> = self
            .grove
            .keys()
            .filter(|c| {
                (c.y == self.position.y && self.forward_idx % 2 == 0)
                    || (c.x == self.position.x && self.forward_idx % 2 == 1)
            })
            .sorted()
            .collect();

        let returning = if self.forward_idx == 0 || self.forward_idx == 1 {
            slice[0].clone()
        } else {
            slice[slice.len() - 1].clone()
        };

        // println!("\n\n\n");
        // dbg!(self.position, self.forward_idx, &slice, returning);

        returning
    }

    fn next_position(&self) -> (Coordinate, Tile) {
        let mut next = self.position + DIRECTIONS[self.forward_idx];
        if !self.grove.contains_key(&next) {
            next = self.wrapped();
        }
        let tile = self.grove.get(&next).expect("");
        (next, *tile)
    }

    fn take_step(&self) -> Option<Coordinate> {
        let (next_pos, tile) = self.next_position();
        match tile {
            Tile::Wall => None,
            Tile::Open => Some(next_pos),
            Tile::None => panic!("unexpected"),
        }
    }

    fn apply_movement(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward { by } => {
                for _ in 0..*by {
                    match self.take_step() {
                        Some(pos) => self.position = pos,
                        None => break,
                    }
                }
            }
            Movement::Turn { to_the } => {
                let change = match to_the {
                    Turn::Left => -1,
                    Turn::Right => 1,
                };
                let base = self.forward_idx as isize + DIRECTIONS.len() as isize + change;
                self.forward_idx = base as usize % DIRECTIONS.len();
            }
        }
    }

    fn password(&self) -> usize {
        let row = self.position.y as usize + 1;
        let column = self.position.x as usize + 1;
        let facing = self.forward_idx;
        println!("1000 * {row} + 4 * {column} + {facing}");

        (1000 * row) + (4 * column) + facing
    }
}

impl Iterator for Walk {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.directions_idx >= self.directions.len() {
            return None;
        }

        let movement = self.directions[self.directions_idx].clone();
        self.apply_movement(&movement);
        self.directions_idx += 1;
        Some(())
    }
}

fn main() {
    let input = input_store::get_input(2022, 22);
    //     let input = r#"        ...#
    //         .#..
    //         #...
    //         ....
    // ...#.......#
    // ........#...
    // ..#....#....
    // ..........#.
    //         ...#....
    //         .....#..
    //         .#......
    //         ......#.

    // 10R5L5R10L4R5L5
    // "#;

    let mut input_split = input.split("\n\n");
    let grove = Grove::from_lines(input_split.next().unwrap());
    let grove: Grove = grove
        .iter()
        .filter(|(c, v)| **v != Tile::None)
        .map(|(c, v)| (c.clone(), v.clone()))
        .collect();
    let (_, directions) = parse_directions(input_split.next().unwrap()).expect("");

    // println!("{grove}");
    // println!("{}", Directions(directions.clone()));

    // let sorted_keys: Vec<&Coordinate> = grove.keys().sorted().collect();
    // dbg!(sorted_keys);

    let mut walk = Walk::new(grove, directions);
    while let Some(_) = walk.next() {
        // println!("{}, {}", walk.position, DIRECTIONS[walk.forward_idx]);
    }

    // let directions = Vec<Vector>

    println!("part_1 => {}", walk.password());
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
