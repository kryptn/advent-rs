use std::collections::HashMap;

use advent::{input_store, parsers::parse_usize};
use advent_toolbox::spatial::{Coordinate, Direction, Space};
use nom::{branch::alt, character::complete::one_of, multi::many1, IResult};

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

#[derive(Eq, PartialEq, Hash)]
struct Edge {
    square: Coordinate,
    side: Direction,
}

impl From<(Coordinate, char)> for Edge {
    fn from((square, d): (Coordinate, char)) -> Self {
        let side = d.into();
        Self { square, side }
    }
}

impl From<(isize, isize, char)> for Edge {
    fn from((x, y, d): (isize, isize, char)) -> Self {
        let square = (x, y).into();
        (square, d).into()
    }
}

#[derive(Debug)]
struct Square {
    inner: Space<Coordinate, Tile>,
    upper: Coordinate,
    // edges: Vec<Edge>
}

#[derive(Debug, Clone)]
struct Cursor {
    square: Coordinate,
    position: Coordinate,
    direction: Direction,
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {}), {:?}",
            self.square.x, self.square.y, self.position.x, self.position.y, self.direction
        )
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

fn parse_turn(input: &str) -> IResult<&str, Instruction> {
    let (input, to_the) = one_of("LR")(input)?;
    let to_the = to_the.into();
    let turn = match to_the {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!(),
    };
    Ok((input, turn))
}

fn parse_forward(input: &str) -> IResult<&str, Instruction> {
    let (input, by) = parse_usize(input)?;
    Ok((input, Instruction::Forward(by)))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((parse_turn, parse_forward)))(input)
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Up, // actually down in puzzle desc
    Direction::Left,
    Direction::Down, // actually up
];

fn dir_idx(direction: Direction) -> usize {
    let (idx, _) = DIRECTIONS
        .iter()
        .enumerate()
        .find(|(_, d)| **d == direction)
        .unwrap();
    idx
}

fn turn(direction: Direction, instruction: &Instruction) -> Direction {
    let idx = dir_idx(direction) + DIRECTIONS.len();
    match instruction {
        Instruction::Left => DIRECTIONS[(idx - 1) % DIRECTIONS.len()],
        Instruction::Right => DIRECTIONS[(idx + 1) % DIRECTIONS.len()],
        _ => panic!(),
    }
}

impl Cursor {
    fn go(
        &self,
        inst: &Instruction,
        squares: &HashMap<Coordinate, Square>,
        edges: &HashMap<Edge, (Edge, bool)>,
    ) -> Self {
        let mut next = self.clone();
        match inst.clone() {
            Instruction::Left | Instruction::Right => next.direction = turn(next.direction, inst),

            Instruction::Forward(by) => {
                for _ in 0..by {
                    let square = squares.get(&next.square).unwrap();
                    let next_pos = next.position + Coordinate::from(next.direction);
                    let tile = square.inner.get(&next_pos).unwrap_or(&Tile::None);

                    match tile {
                        Tile::Wall => {
                            break;
                        }
                        Tile::Open => next.position = next_pos,
                        Tile::None => {
                            let edge = Edge {
                                square: next.square,
                                side: next.direction,
                            };
                            let from_zero = match next.direction {
                                Direction::Up | Direction::Down => next.position.x,
                                Direction::Right | Direction::Left => next.position.y,
                                Direction::None => panic!(),
                            };

                            let (dest, invert) = edges.get(&edge).unwrap();

                            let from_zero = if *invert { 49 - from_zero } else { from_zero };

                            let square = squares.get(&dest.square).unwrap();
                            let (position, direction): (Coordinate, Direction) = match dest.side {
                                Direction::Up => {
                                    ((from_zero, square.upper.y).into(), Direction::Down)
                                }
                                Direction::Right => {
                                    ((square.upper.x, from_zero).into(), Direction::Left)
                                }
                                Direction::Down => ((from_zero, 0).into(), Direction::Up),
                                Direction::Left => ((0, from_zero).into(), Direction::Right),
                                Direction::None => panic!(),
                            };

                            let square = squares.get(&dest.square).unwrap();
                            let tile = square.inner.get(&position).unwrap_or(&Tile::None);
                            if *tile == Tile::Open {
                                next.position = position;
                                next.square = dest.square;
                                next.direction = direction;
                            } else {
                                break;
                            }
                        }
                    }
                    // println!("{}", printable_space(&next, &squares));
                    // std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
        }

        next
    }

    fn score(&self, upper: Coordinate) -> isize {
        let raw_x = self.position.x + (self.square.x * (upper.x + 1)) + 1;
        let raw_y = self.position.y + (self.square.y * (upper.y + 1)) + 1;

        (raw_y * 1000) + (raw_x * 4) + dir_idx(self.direction) as isize
    }
}

fn get_squares(input: &str) -> HashMap<Coordinate, Square> {
    let side_len = input
        .lines()
        .map(|l| l.len())
        .filter(|l| l > &0)
        .min()
        .unwrap();

    let tiles = Space::from_lines(input);

    let (_, upper) = tiles.bounding_box();

    let x_segments = upper.x / side_len as isize;
    let y_segments = upper.y / side_len as isize;

    let mut out = HashMap::new();

    for y_offset in 0..=y_segments {
        for x_offset in 0..=x_segments {
            let base: Coordinate =
                (x_offset * side_len as isize, y_offset * side_len as isize).into();

            if tiles.contains_key(&base) {
                let mut inner: Space<Coordinate, Tile> = Space::new();
                for y in 0..50 {
                    for x in 0..50 {
                        let coord = (x, y).into();
                        let tile_coord = coord + base;
                        // dbg!(base, coord, tile_coord);
                        inner.insert(coord, tiles[&tile_coord]);
                    }
                }

                let (_, upper) = inner.bounding_box();

                let offset = (x_offset, y_offset).into();
                let square = Square { inner, upper };

                out.insert(offset, square);
            }
        }
    }

    out
}

// gross and hardcoded.
const A: Coordinate = Coordinate { x: 1, y: 0 };
const B: Coordinate = Coordinate { x: 2, y: 0 };
const C: Coordinate = Coordinate { x: 1, y: 1 };
const D: Coordinate = Coordinate { x: 0, y: 2 };
const E: Coordinate = Coordinate { x: 1, y: 2 };
const F: Coordinate = Coordinate { x: 0, y: 3 };

fn p1_edges() -> HashMap<Edge, (Edge, bool)> {
    vec![
        ((A, 'u').into(), ((C, 'd').into(), false)),
        ((A, 'r').into(), ((B, 'l').into(), false)),
        ((A, 'd').into(), ((E, 'u').into(), false)),
        ((A, 'l').into(), ((B, 'r').into(), false)),
        //
        ((B, 'u').into(), ((B, 'd').into(), false)),
        ((B, 'r').into(), ((A, 'l').into(), false)),
        ((B, 'd').into(), ((B, 'u').into(), false)),
        ((B, 'l').into(), ((A, 'r').into(), false)),
        //
        ((C, 'u').into(), ((E, 'd').into(), false)),
        ((C, 'r').into(), ((C, 'l').into(), false)),
        ((C, 'd').into(), ((A, 'u').into(), false)),
        ((C, 'l').into(), ((C, 'r').into(), false)),
        //
        ((D, 'u').into(), ((F, 'd').into(), false)),
        ((D, 'r').into(), ((E, 'l').into(), false)),
        ((D, 'd').into(), ((F, 'u').into(), false)),
        ((D, 'l').into(), ((E, 'r').into(), false)),
        //
        ((E, 'u').into(), ((A, 'd').into(), false)),
        ((E, 'r').into(), ((D, 'l').into(), false)),
        ((E, 'd').into(), ((C, 'u').into(), false)),
        ((E, 'l').into(), ((D, 'r').into(), false)),
        //
        ((F, 'u').into(), ((D, 'd').into(), false)),
        ((F, 'r').into(), ((F, 'l').into(), false)),
        ((F, 'd').into(), ((D, 'u').into(), false)),
        ((F, 'l').into(), ((F, 'r').into(), false)),
    ]
    .into_iter()
    .collect()
}

fn p2_edges() -> HashMap<Edge, (Edge, bool)> {
    vec![
        ((A, 'u').into(), ((C, 'd').into(), false)),
        ((A, 'r').into(), ((B, 'l').into(), false)),
        ((A, 'd').into(), ((F, 'l').into(), false)),
        ((A, 'l').into(), ((D, 'l').into(), true)),
        //
        ((B, 'u').into(), ((C, 'r').into(), false)),
        ((B, 'r').into(), ((E, 'r').into(), true)),
        ((B, 'd').into(), ((F, 'u').into(), false)),
        ((B, 'l').into(), ((A, 'r').into(), false)),
        //
        ((C, 'u').into(), ((E, 'd').into(), false)),
        ((C, 'r').into(), ((B, 'u').into(), false)),
        ((C, 'd').into(), ((A, 'u').into(), false)),
        ((C, 'l').into(), ((D, 'd').into(), false)),
        //
        ((D, 'u').into(), ((F, 'd').into(), false)),
        ((D, 'r').into(), ((E, 'l').into(), false)),
        ((D, 'd').into(), ((C, 'l').into(), false)),
        ((D, 'l').into(), ((A, 'l').into(), true)),
        //
        ((E, 'u').into(), ((F, 'r').into(), false)),
        ((E, 'r').into(), ((B, 'r').into(), true)),
        ((E, 'd').into(), ((C, 'u').into(), false)),
        ((E, 'l').into(), ((D, 'r').into(), false)),
        //
        ((F, 'u').into(), ((B, 'd').into(), false)),
        ((F, 'r').into(), ((E, 'u').into(), false)),
        ((F, 'd').into(), ((D, 'u').into(), false)),
        ((F, 'l').into(), ((A, 'd').into(), false)),
    ]
    .into_iter()
    .collect()
}

fn solve_with(
    squares: &HashMap<Coordinate, Square>,
    edges: &HashMap<Edge, (Edge, bool)>,
    instructions: &Vec<Instruction>,
) -> Cursor {
    let mut cursor = Cursor {
        square: (1, 0).into(),
        position: (0, 0).into(),
        direction: Direction::Right,
    };

    for inst in instructions {
        cursor = cursor.go(inst, &squares, &edges);
    }

    cursor
}

fn main() {
    let input = input_store::get_input(2022, 22);

    let split: Vec<&str> = input.split("\n\n").collect();

    let squares = get_squares(split[0]);
    let (_, instructions) = parse_directions(split[1].trim()).unwrap();

    let edges = p1_edges();
    let cursor = solve_with(&squares, &edges, &instructions);
    let p1 = cursor.score((49, 49).into());
    println!("part_1 => {}", p1);

    let edges = p2_edges();
    let cursor = solve_with(&squares, &edges, &instructions);
    let p2 = cursor.score((49, 49).into());
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
