use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

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

pub trait Traversable<P: Point> {
    fn connected(&self, start: &P, end: &P) -> bool;
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

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

    pub fn zip_cardinals(&self) -> [(Self, Self); 4] {
        [
            (*self, self.up()),
            (*self, self.right()),
            (*self, self.down()),
            (*self, self.left()),
        ]
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
        let lines_num = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let coord = (x, lines_num - y).into();
                let value = V::from(ch);
                out.push((coord, value))
            }
        }

        out.into_iter().collect()
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
    type Output = Coordinate;

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

    for y in y_left..y_right {
        for x in x_left..x_right {
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

#[cfg(test)]
mod test {
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
}
