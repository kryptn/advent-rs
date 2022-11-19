use std::{
    collections::{HashMap, HashSet},
    iter::Sum,
    ops::{Add, Range},
    slice::Iter,
    str::FromStr,
    sync::{Arc, Mutex, MutexGuard, RwLock},
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub enum Axis {
    X(i32),
    Y(i32),
}

impl Axis {
    pub fn new(along: char, value: i32) -> Self {
        match along {
            'x' => Axis::X(value),
            'y' => Axis::Y(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl FromStr for CardinalDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "N" => Ok(Self::North),
            "R" | "E" => Ok(Self::East),
            "D" | "S" => Ok(Self::South),
            "L" | "W" => Ok(Self::West),
            _ => Err(anyhow::Error::msg("unknown cardinal direction")),
        }
    }
}

impl From<&str> for CardinalDirection {
    fn from(dir_str: &str) -> Self {
        Self::from_str(dir_str).unwrap()
    }
}

impl From<char> for CardinalDirection {
    fn from(dir_char: char) -> Self {
        Self::from_str(&dir_char.to_string()).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

pub enum RelativeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for RelativeDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "N" => Ok(Self::Up),
            "R" | "E" => Ok(Self::Right),
            "D" | "S" => Ok(Self::Down),
            "L" | "W" => Ok(Self::Left),
            _ => Err(anyhow::Error::msg("unknown relative direction")),
        }
    }
}

impl From<&str> for RelativeDirection {
    fn from(dir_str: &str) -> Self {
        Self::from_str(dir_str).unwrap()
    }
}

impl From<char> for RelativeDirection {
    fn from(dir_char: char) -> Self {
        Self::from_str(&dir_char.to_string()).unwrap()
    }
}

#[cfg(feature = "parse")]
pub fn parse_cardinal(input: &str) -> nom::IResult<&str, CardinalDirection> {
    let (input, dir) = nom::character::complete::one_of("NSEW")(input)?;
    Ok((input, dir.into()))
}

#[cfg(feature = "parse")]
pub fn parse_relative(input: &str) -> nom::IResult<&str, RelativeDirection> {
    let (input, dir) = nom::character::complete::one_of("UDLR")(input)?;
    Ok((input, dir.into()))
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Coordinate::new(self.x - 1, self.y - 1),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x + 1, self.y - 1),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn grid_around(&self) -> Vec<Self> {
        vec![
            Coordinate::new(self.x - 1, self.y - 1),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x + 1, self.y - 1),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x, self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn turn(&self, dir: RelativeDirection) -> Self {
        match dir {
            RelativeDirection::Right => Self {
                x: self.y,
                y: self.x * -1,
            },
            RelativeDirection::Left => Self {
                x: self.y * -1,
                y: self.x,
            },
            RelativeDirection::Up => self.clone(),
            RelativeDirection::Down => self.scale(-1),
        }
    }

    pub fn mirror(&self, axis: Axis) -> Self {
        match axis {
            Axis::X(v) => (self.x + ((v - self.x) * 2), self.y).into(),
            Axis::Y(v) => (self.x, self.y + ((v - self.y) * 2)).into(),
        }
    }
}

impl From<CardinalDirection> for Coordinate {
    fn from(d: CardinalDirection) -> Self {
        match d {
            CardinalDirection::North => Self::new(0, 1),
            CardinalDirection::East => Self::new(1, 0),
            CardinalDirection::South => Self::new(0, -1),
            CardinalDirection::West => Self::new(-1, 0),
        }
    }
}

impl From<RelativeDirection> for Coordinate {
    fn from(d: RelativeDirection) -> Self {
        match d {
            RelativeDirection::Up => Self::new(0, 1),
            RelativeDirection::Right => Self::new(1, 0),
            RelativeDirection::Down => Self::new(0, -1),
            RelativeDirection::Left => Self::new(-1, 0),
        }
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(t: (i32, i32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<&str> for Coordinate {
    fn from(input: &str) -> Self {
        let mut input_split = input.trim().split(",");
        let x = input_split.next().unwrap().parse().unwrap();
        let y = input_split.next().unwrap().parse().unwrap();
        (x, y).into()
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sum<Coordinate> for Coordinate {
    fn sum<I: Iterator<Item = Coordinate>>(iter: I) -> Self {
        iter.fold(Self::new(0, 0), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Coordinate> for Coordinate {
    fn sum<I: Iterator<Item = &'a Coordinate>>(iter: I) -> Self {
        iter.fold(Self::new(0, 0), |a, b| a + *b)
    }
}

impl Coordinate {
    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn scale(self, by: i32) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
        }
    }

    pub fn cardinals(self) -> Vec<Self> {
        vec![self.up(), self.right(), self.down(), self.left()]
    }

    pub fn weighted_cardinals(self, measure: impl Fn(&Coordinate) -> i32) -> Vec<Self> {
        self.cardinals()
            .iter()
            .sorted_by(|&l, &r| measure(l).cmp(&measure(r)))
            .cloned()
            .collect()
    }
}

pub fn coordinate_str(given: &str, sep: &str) -> Coordinate {
    let it: Vec<&str> = given.split(sep).collect();
    Coordinate {
        x: it[0].parse::<i32>().unwrap(),
        y: it[1].parse::<i32>().unwrap(),
    }
}

fn sorted<T: PartialEq + PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn coordinates_within(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    let mut coords: Vec<Coordinate> = Vec::new();

    let (y_left, y_right) = sorted(a.y, b.y);
    let (x_left, x_right) = sorted(a.x, b.x);

    for y in y_left..=y_right {
        for x in x_left..=x_right {
            coords.push(Coordinate::new(x, y))
        }
    }

    coords
}

pub fn line_between(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    if a.x == b.x || a.y == b.y {
        return coordinates_within(a, b);
    }

    let mut cursor = a;
    let mut coords = vec![a];

    let x_delta = if a.x > b.x {
        RelativeDirection::Left
    } else {
        RelativeDirection::Right
    };

    let y_delta = if a.y > b.y {
        RelativeDirection::Down
    } else {
        RelativeDirection::Up
    };

    while cursor != b {
        cursor = cursor + x_delta.into() + y_delta.into();
        coords.push(cursor);
    }

    coords
}

pub fn iter_rows(a: Coordinate, b: Coordinate) -> Vec<Vec<Coordinate>> {
    let (y_left, y_right) = sorted(a.y, b.y);
    let (x_left, x_right) = sorted(a.x, b.x);

    let mut lines = Vec::new();

    for y in y_left..=y_right {
        let mut row = Vec::new();
        for x in x_left..=x_right {
            row.push(Coordinate::new(x, y))
        }

        lines.push(row);
    }

    lines
}

pub fn manhattan(a: Coordinate, b: Coordinate) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

pub type Grid<T> = HashMap<Coordinate, T>;

pub fn new_with_size<T: PartialEq + PartialOrd + Default>(x: i32, y: i32) -> Grid<T> {
    let mut g = Grid::<T>::new();
    for coordinate in coordinates_within(Coordinate::new(0, 0), Coordinate::new(x, y)) {
        g.insert(coordinate, T::default());
    }

    g
}

pub fn bounding_box<T>(grid: &Grid<T>) -> (Coordinate, Coordinate) {
    let mut lower = Coordinate::new(0, 0);
    let mut upper = Coordinate::new(0, 0);

    for coordinate in grid.keys() {
        if coordinate.x < lower.x {
            lower.x = coordinate.x;
        }
        if coordinate.y < lower.y {
            lower.y = coordinate.y
        }

        if coordinate.x > upper.x {
            upper.x = coordinate.x;
        }
        if coordinate.y > upper.y {
            upper.y = coordinate.y
        }
    }

    (lower, upper)
}

pub fn print_grid<T>(g: &Grid<T>)
where
    T: Default + std::fmt::Display + Clone,
{
    let (lower, upper) = bounding_box(&g);

    for row in iter_rows(lower, upper) {
        for coord in row {
            let item = match g.get(&coord) {
                Some(i) => i.clone(),
                None => T::default(),
            };
            print!("{}", item);
        }
        print!("\n");
    }
    println!("");
}

pub trait Passable {
    fn is_passable(&self) -> bool;
}

#[derive(Clone, Debug)]
pub enum Traversal {
    Found(Vec<Coordinate>),
    NoPath(Vec<Coordinate>),
}

fn inner_traverse_astar<T: Passable>(
    grid: &Grid<T>,
    position: Coordinate,
    goal: Coordinate,
    visited: Vec<Coordinate>,
) -> Vec<Traversal> {
    if position == goal {
        let mut visited = visited;
        visited.push(position);
        return vec![Traversal::Found(visited)];
    }

    let sorted_cardinals = position.weighted_cardinals(|c| {
        let tc = c.clone();
        manhattan(tc, goal)
    });

    let valid_coordinates = sorted_cardinals
        .iter()
        .filter(|&c| {
            if let Some(item) = grid.get(c) {
                item.is_passable()
            } else {
                false
            }
        })
        .filter(|&c| !visited.contains(c));

    let results: Vec<Traversal> = valid_coordinates
        .cloned()
        .map(|c| {
            let mut visited = visited.clone();
            visited.push(c);

            inner_traverse_astar(grid, c, goal, visited)
        })
        .flatten()
        .collect();

    if results.len() == 0 {
        vec![Traversal::NoPath(visited)]
    } else {
        results
    }
}

pub fn traverse_astar<T: Passable>(
    grid: &Grid<T>,
    start: Coordinate,
    goal: Coordinate,
) -> Vec<Traversal> {
    let visited = vec![];
    inner_traverse_astar(grid, start, goal, visited)
}

pub fn shortest_path<T: Passable>(
    grid: &Grid<T>,
    start: Coordinate,
    goal: Coordinate,
) -> Option<Vec<Coordinate>> {
    let paths = traverse_astar(&grid, start, goal);
    let sorted_paths: Vec<Vec<Coordinate>> = paths
        .iter()
        .filter(|&p| matches!(p, Traversal::Found(_)))
        .map(|p| match p {
            Traversal::Found(fp) => fp.clone(),
            Traversal::NoPath(_) => unreachable!(),
        })
        .sorted_by(|l, r| l.len().cmp(&r.len()))
        .collect();

    if !sorted_paths.is_empty() {
        Some(sorted_paths.iter().nth(0).unwrap().clone())
    } else {
        None
    }
}

pub fn from_text<T>(input: &str) -> Result<Grid<T>>
where
    T: FromStr + Default,
{
    let mut out = Grid::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, value) in line.trim().chars().enumerate() {
            let value = value.to_string().parse::<T>().unwrap_or_default();
            out.insert((x as i32, y as i32).into(), value);
        }
    }

    Ok(out)
}

struct NewGrid<T>(HashMap<Coordinate, T>);

impl<T> std::ops::Deref for NewGrid<T> {
    type Target = HashMap<Coordinate, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for NewGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> NewGrid<T> {
    fn new() -> Self {
        NewGrid(HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_newgrid() {
        let mut grid: NewGrid<i32> = NewGrid::new();
        grid.insert((0, 0).into(), 3);
    }

    #[test]
    fn test_range() {
        let coords = coordinates_within(Coordinate::new(0, 0), Coordinate::new(1, 1));
        let expected = vec![
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(0, 1),
            Coordinate::new(1, 1),
        ];

        assert_eq!(coords, expected);
    }

    #[test]
    fn test_fromdir() {
        let up_dir: CardinalDirection = "U".into();
        let north_coord: Coordinate = up_dir.into();

        assert_eq!(north_coord, Coordinate::new(0, 1));
    }

    #[test]
    fn test_turn_right() {
        let mut heading = Coordinate::new(0, 1);

        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(1, 0));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(0, -1));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(-1, 0));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(0, 1));
    }

    #[test]
    fn test_turn_left() {
        let mut heading = Coordinate::new(0, 1);

        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(-1, 0));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(0, -1));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(1, 0));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(0, 1));
    }

    #[test]
    fn test_with_sep() {
        let it = coordinate_str("1,2", ",");
        assert_eq!(it, Coordinate::new(1, 2));
    }

    #[test]
    fn test_weighted_coordinate() {
        let anchor = Coordinate::zero();
        let dirs = Coordinate::new(12, -5).weighted_cardinals(|c| manhattan(anchor, c.clone()));

        println!("{:?}", dirs);
    }

    mod test_grid {

        use crate::grid::*;

        pub struct Cell {
            coordinate: Coordinate,
        }

        impl Passable for Cell {
            fn is_passable(&self) -> bool {
                0 <= self.coordinate.y && self.coordinate.y < 5
            }
        }

        pub fn new_grid(x: i32, y: i32) -> Grid<Cell> {
            let mut grid = Grid::new();
            for coordinate in coordinates_within(Coordinate::zero(), Coordinate::new(x, y)) {
                grid.insert(coordinate, Cell { coordinate });
            }

            grid
        }
    }

    #[test]
    fn test_astar() {
        let grid = test_grid::new_grid(10, 10);

        let start = Coordinate::new(2, 4);
        let goal = Coordinate::new(8, 2);

        let p = traverse_astar(&grid, start, goal);
        dbg!(p);
    }

    #[test]
    fn coord_line() {
        let a = (0, 0).into();
        let b = (2, 2).into();

        let t = line_between(a, b);
        let given = vec![(0, 0).into(), (1, 1).into(), (2, 2).into()];

        assert_eq!(t, given);
    }
}
