use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use itertools::{Itertools, TakeWhileInclusive};

pub trait Point {}

#[derive(Debug)]
pub struct Space<P, T>(HashMap<P, T>)
where
    P: Point;

impl<P: Point, T> std::ops::Deref for Space<P, T> {
    type Target = HashMap<P, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Point, T> std::ops::DerefMut for Space<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Point, T> Space<P, T> {
    pub fn new() -> Self {
        Space(HashMap::new())
    }
}

impl<P, T> From<Vec<(P, T)>> for Space<P, T>
where
    P: Point + Hash + Eq,
{
    fn from(input: Vec<(P, T)>) -> Self {
        Self(input.into_iter().collect())
    }
}

pub trait Traversable {
    fn is_traversable(&self) -> bool;
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

pub const UP: Coordinate = Coordinate { x: 0, y: 1 };
pub const DOWN: Coordinate = Coordinate { x: 0, y: -1 };
pub const LEFT: Coordinate = Coordinate { x: -1, y: 0 };
pub const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        (self.x, self.y + 1).into()
    }

    pub fn right(&self) -> Self {
        (self.x + 1, self.y).into()
    }

    pub fn down(&self) -> Self {
        (self.x, self.y - 1).into()
    }

    pub fn left(&self) -> Self {
        (self.x - 1, self.y).into()
    }

    pub fn cardinals(&self) -> [Self; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }

    pub fn neighbors(&self) -> [Self; 8] {
        [
            self.up(),
            self.up().right(),
            self.right(),
            self.right().down(),
            self.down(),
            self.down().left(),
            self.left(),
            self.left().up(),
        ]
    }

    pub fn zip_cardinals(&self) -> [(Self, Self); 4] {
        [
            (*self, self.up()),
            (*self, self.right()),
            (*self, self.down()),
            (*self, self.left()),
        ]
    }

    pub fn distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    pub fn folded_at(&self, line: &Self) -> Self {
        assert!(line.x == 0 || line.y == 0);
        if line.x > 0 {
            let pivot = line.x;
            if self.x <= pivot {
                *self
            } else {
                let half: isize = self.x - pivot;
                (self.x - half * 2 + 1, self.y).into()
            }
        } else {
            let pivot = line.y;
            if self.y <= pivot {
                *self
            } else {
                let half = self.y - pivot;
                (self.x, self.y - half * 2 + 1).into()
            }
        }
    }

    pub fn normalize(&self) -> Self {
        let x = if self.x > 0 {
            1
        } else if self.x < 0 {
            -1
        } else {
            0
        };
        let y = if self.y > 0 {
            1
        } else if self.y < 0 {
            -1
        } else {
            0
        };

        (x, y).into()
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        ((self.x + rhs.x), (self.y + rhs.y)).into()
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        ((self.x - rhs.x), (self.y - rhs.y)).into()
    }
}

impl Mul<isize> for Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: isize) -> Self::Output {
        (self.x * rhs, self.y * rhs).into()
    }
}

macro_rules! coord_from {
    ( $x:ty ) => {
        impl From<($x, $x)> for Coordinate {
            fn from((x, y): ($x, $x)) -> Self {
                let x = x as isize;
                let y = y as isize;
                Self { x, y }
            }
        }
    };
}

coord_from!(usize);
coord_from!(isize);
coord_from!(u32);
coord_from!(i32);
coord_from!(u64);
coord_from!(i64);

impl Point for Coordinate {}

impl Point for (isize, isize) {}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coordinate3d {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn right(&self) -> Self {
        (self.x + 1, self.y, self.z).into()
    }

    pub fn left(&self) -> Self {
        (self.x - 1, self.y, self.z).into()
    }

    pub fn up(&self) -> Self {
        (self.x, self.y + 1, self.z).into()
    }

    pub fn down(&self) -> Self {
        (self.x, self.y - 1, self.z).into()
    }

    pub fn forward(&self) -> Self {
        (self.x, self.y, self.z + 1).into()
    }

    pub fn backward(&self) -> Self {
        (self.x, self.y, self.z - 1).into()
    }

    pub fn cardinals(&self) -> [Self; 6] {
        [
            self.up(),
            self.right(),
            self.down(),
            self.left(),
            self.forward(),
            self.backward(),
        ]
    }

    pub fn zip_cardinals(&self) -> [(Self, Self); 6] {
        [
            (*self, self.up()),
            (*self, self.right()),
            (*self, self.down()),
            (*self, self.left()),
            (*self, self.forward()),
            (*self, self.backward()),
        ]
    }
}

impl std::fmt::Display for Coordinate3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Coordinate3d> for Coordinate3d {
    type Output = Self;

    fn add(self, rhs: Coordinate3d) -> Self::Output {
        ((self.x + rhs.x), (self.y + rhs.y), (self.z + rhs.z)).into()
    }
}

impl Sub<Coordinate3d> for Coordinate3d {
    type Output = Self;

    fn sub(self, rhs: Coordinate3d) -> Self::Output {
        ((self.x - rhs.x), (self.y - rhs.y), (self.z - rhs.z)).into()
    }
}

macro_rules! coord3d_from {
    ( $x:ty ) => {
        impl From<($x, $x, $x)> for Coordinate3d {
            fn from((x, y, z): ($x, $x, $x)) -> Self {
                let x = x as isize;
                let y = y as isize;
                let z = z as isize;
                Self { x, y, z }
            }
        }
    };
}

coord3d_from!(usize);
coord3d_from!(isize);
coord3d_from!(u32);
coord3d_from!(i32);
coord3d_from!(u64);
coord3d_from!(i64);

impl Point for Coordinate3d {}

impl Point for (isize, isize, isize) {}

impl<K: Point, V> FromIterator<(K, V)> for Space<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Space<K, V> {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        Self(map)
    }
}

impl<V> From<&str> for Space<Coordinate, V>
where
    V: From<char>,
{
    fn from(input: &str) -> Self {
        let mut cells = Vec::new();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, value) in line.trim().chars().enumerate() {
                cells.push(((x, y).into(), value.into()))
            }
        }

        cells.into_iter().collect()
    }
}

impl<V> std::str::FromStr for Space<Coordinate, V>
where
    V: From<char>,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<V> From<&String> for Space<Coordinate, V>
where
    V: From<char>,
{
    fn from(input: &String) -> Self {
        Self::from(input.as_str())
    }
}

impl<V> From<String> for Space<Coordinate, V>
where
    V: From<char>,
{
    fn from(input: String) -> Self {
        Self::from(input.as_str())
    }
}

impl<V> Space<Coordinate, V> {
    pub fn bounding_box(&self) -> (Coordinate, Coordinate) {
        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();

        for key in self.keys() {
            x_set.insert(key.x);
            y_set.insert(key.y);
        }

        let x: Vec<isize> = x_set.into_iter().sorted().collect();
        let y: Vec<isize> = y_set.into_iter().sorted().collect();

        ((x[0], y[0]).into(), (x[x.len() - 1], y[y.len() - 1]).into())
    }

    pub fn from_lines(input: &str) -> Self
    where
        V: From<char>,
    {
        let mut out = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let coord = (x, y).into();
                let value = V::from(ch);
                out.push((coord, value))
            }
        }

        out.into_iter().collect()
    }

    pub fn from_lines_rev(input: &str) -> Self
    where
        V: From<char>,
    {
        let mut out = Vec::new();
        for (y, line) in input.lines().rev().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let coord = (x, y).into();
                let value = V::from(ch);
                out.push((coord, value))
            }
        }

        out.into_iter().collect()
    }

    pub fn a_star(&self, start: &Coordinate, goal: &Coordinate) -> Option<Vec<Coordinate>>
    where
        V: Traversable,
    {
        let mut frontier = VecDeque::new();
        frontier.push_back(*start);

        let mut came_from = HashMap::new();
        came_from.insert(*start, *start);

        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(*start, 0);

        while let Some(current) = frontier.pop_front() {
            if current == *goal {
                break;
            }

            for next in current.cardinals().iter() {
                if let Some(value) = self.get(next) {
                    if !value.is_traversable() {
                        continue;
                    }
                } else {
                    continue;
                }

                let new_cost = cost_so_far[&current] + 1;
                if !cost_so_far.contains_key(next) || new_cost < cost_so_far[next] {
                    cost_so_far.insert(*next, new_cost);
                    let priority = new_cost + next.distance(goal);
                    frontier.push_back(*next);
                    came_from.insert(*next, current);
                }
            }
        }

        let mut current = *goal;
        let mut path = vec![current];
        while current != *start {
            current = came_from[&current];
            path.push(current);
        }
        path.reverse();

        Some(path)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = (Coordinate, &V)>> {
        let (lower, upper) = self.bounding_box();
        let mut out = Vec::new();
        for y in lower.y..=upper.y {
            let mut row = Vec::new();
            for x in lower.x..=upper.x {
                let coord = (x, y).into();
                if let Some(value) = self.get(&coord) {
                    row.push((coord, value));
                }
            }
            out.push(row.into_iter());
        }
        out.into_iter()
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = (Coordinate, &V)>> {
        let (lower, upper) = self.bounding_box();
        let mut out = Vec::new();
        for x in lower.x..=upper.x {
            let mut columns = Vec::new();
            for y in lower.y..=upper.y {
                let coord = (x, y).into();
                if let Some(value) = self.get(&coord) {
                    columns.push((coord, value));
                }
            }
            out.push(columns.into_iter());
        }
        out.into_iter()
    }

    pub fn x_bounds(&self) -> (isize, isize) {
        let (lower, upper) = self.bounding_box();
        (lower.x, upper.x)
    }

    pub fn y_bounds(&self) -> (isize, isize) {
        let (lower, upper) = self.bounding_box();
        (lower.y, upper.y)
    }

    pub fn bisect_at(&self, line: &Coordinate) -> (Self, Self)
    where
        V: Clone,
    {
        let mut left_or_top = Self::new();
        let mut right_or_bottom = Self::new();

        for (coord, value) in self.iter() {
            if (line.x > 0 && coord.x > line.x) || (line.y > 0 && coord.y > line.y) {
                right_or_bottom.insert(*coord, value.clone());
            } else {
                left_or_top.insert(*coord, value.clone());
            }
        }

        (left_or_top, right_or_bottom)
    }
}

impl<V> std::fmt::Display for Space<Coordinate, V>
where
    V: std::fmt::Display + Default + Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let (lower, upper) = self.bounding_box();

        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let coord = (x, y).into();
                let item = self.get(&coord).unwrap_or(&V::default()).clone();
                out.push_str(&format!("{item}"));
            }
            out.push('\n');
        }
        write!(f, "{out}")
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Cardinal {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Add<Cardinal> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Cardinal) -> Self::Output {
        match rhs {
            Cardinal::North => self.up(),
            Cardinal::NorthEast => self.up().right(),
            Cardinal::East => self.right(),
            Cardinal::SouthEast => self.down().right(),
            Cardinal::South => self.down(),
            Cardinal::SouthWest => self.down().left(),
            Cardinal::West => self.left(),
            Cardinal::NorthWest => self.up().left(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl Add<Direction> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => self.up(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::None => self,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' | 'U' | 'u' => Self::Up,
            '>' | 'R' | 'r' => Self::Right,
            'v' | 'D' | 'd' => Self::Down,
            '<' | 'L' | 'l' => Self::Left,
            _ => Self::None,
        }
    }
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::None => *self,
        }
    }
    pub fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::None => *self,
        }
    }
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0isize, 1).into(),
            Direction::Right => (1isize, 0).into(),
            Direction::Down => (0isize, -1).into(),
            Direction::Left => (-1isize, 0).into(),
            Direction::None => (0isize, 0).into(),
        }
    }
}

impl<V> Space<Coordinate3d, V> {
    pub fn bounding_box(&self) -> (Coordinate3d, Coordinate3d) {
        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();
        let mut z_set = HashSet::new();

        for key in self.keys() {
            x_set.insert(key.x);
            y_set.insert(key.y);
            z_set.insert(key.z);
        }

        let x: Vec<isize> = x_set.into_iter().sorted().collect();
        let y: Vec<isize> = y_set.into_iter().sorted().collect();
        let z: Vec<isize> = z_set.into_iter().sorted().collect();

        (
            (x[0], y[0], z[0]).into(),
            (x[x.len() - 1], y[y.len() - 1], z[z.len() - 1]).into(),
        )
    }

    pub fn slices(&self) -> Vec<Space<Coordinate, V>>
    where
        V: Clone,
    {
        let mut out = Vec::new();
        let (lower, upper) = self.bounding_box();

        for z in lower.z..=upper.z {
            let mut slice = Vec::new();

            for y in lower.y..=upper.y {
                for x in lower.x..=upper.x {
                    let this = (x, y, z).into();
                    if let Some(value) = self.get(&this) {
                        let coord = (x, y).into();
                        slice.push((coord, value.clone()))
                    }
                }
            }
            out.push(slice.into_iter().collect());
        }

        out
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

impl<V> std::fmt::Display for Space<Coordinate3d, V>
where
    V: std::fmt::Display + Default + Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let planes = self
            .slices()
            .iter()
            .enumerate()
            .map(|(z, s)| format!("z: {z}\n{s}\n---"))
            .join("\n\n");
        write!(f, "{planes}")
    }
}

impl<P, V> Space<P, V>
where
    P: Point + Clone,
{
    pub fn find(&self, f: impl Fn(&V) -> bool) -> Option<P> {
        for (k, v) in self.iter() {
            if f(v) {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn find_all(&self, f: impl Fn(&V) -> bool) -> Vec<P> {
        let mut out = Vec::new();
        for (k, v) in self.iter() {
            if f(v) {
                out.push(k.clone());
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn create_grid() {
        let objects = vec![((2, 4), false)];
        let g: Space<(isize, isize), bool> = objects.into_iter().collect();
        dbg!(g);
    }

    #[test]
    fn test_coordinate_intos() {
        let expected = Coordinate::new(1, 1);
        let c: Coordinate = (1 as usize, 1 as usize).into();
        assert_eq!(c, expected);

        let c: Coordinate = (1 as isize, 1 as isize).into();
        assert_eq!(c, expected);

        let c: Coordinate = (1 as u32, 1 as u32).into();
        assert_eq!(c, expected);

        let c: Coordinate = (1 as isize, 1 as isize).into();
        assert_eq!(c, expected);
    }

    #[rstest]
    #[case((4, 4), (2, 0), (1, 4))]
    #[case((4, 4), (0, 2), (4, 1))]
    #[case((4, 4), (1, 0), (-1, 4))]
    #[case((4, 4), (3, 0), (3, 4))]
    #[case((1, 4), (2, 0), (1, 4))]
    #[case((1, 4), (0, 2), (1, 1))]
    fn test_coordinate_fold(
        #[case] given: (isize, isize),
        #[case] line: (isize, isize),
        #[case] expected: (isize, isize),
    ) {
        let given: Coordinate = given.into();
        let line = line.into();
        let expected = expected.into();
        assert_eq!(given.folded_at(&line), expected);
    }

    #[rstest]
    #[case(0..=5, 2, 0..=2, 3..=5)]
    fn test_bisect(
        #[case] given: impl Iterator<Item = isize>,
        #[case] line: isize,
        #[case] expected_left: impl Iterator<Item = isize>,
        #[case] expected_right: impl Iterator<Item = isize>,
    ) {
        let given: Space<Coordinate, bool> =
            given.map(|x| (x, 0).into()).map(|c| (c, true)).collect();
        let line = (line, 0).into();
        let (a, b) = given.bisect_at(&line);
        let a_keys: HashSet<isize> = a.keys().map(|c| c.x).collect();
        let b_keys: HashSet<isize> = b.keys().map(|c| c.x).collect();
        dbg!(&a_keys, &b_keys);
        let expected_left: HashSet<_> = expected_left.collect();
        let expected_right: HashSet<_> = expected_right.collect();
        assert_eq!(a_keys, expected_left);
        assert_eq!(b_keys, expected_right);
    }
}
