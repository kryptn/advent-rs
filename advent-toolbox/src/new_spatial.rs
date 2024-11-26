use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::{self, IndexMut},
    slice::SliceIndex,
};

fn sorted<T: Ord>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
pub trait Traversable<K>
where
    K: Point,
{
    type State;
    fn is_traversable(&self, from: &K, to: &K) -> bool;
}

pub trait Point
where
    Self: Hash + Eq + Copy + Clone + PartialOrd + PartialEq,
    Self: ops::Add<Output = Self> + ops::AddAssign,
    Self: ops::Sub<Output = Self> + ops::SubAssign,
{
    fn range(&self, other: &Self) -> impl Iterator<Item = Self>;
}

pub trait IntoNeighbors {
    fn neighbors(&self) -> impl Iterator<Item = Self>;
}

impl IntoNeighbors for Coordinate {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let mut neighbors = vec![];

        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                if x == self.x && y == self.y {
                    continue;
                }

                neighbors.push(Self { x, y });
            }
        }

        neighbors.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Point for Coordinate {
    fn range(&self, other: &Self) -> impl Iterator<Item = Self> {
        let (y_left, y_right) = sorted(self.y, other.y);
        let (x_left, x_right) = sorted(self.x, other.x);

        (y_left..=y_right).flat_map(move |y| (x_left..=x_right).map(move |x| Self { x, y }))
    }
}

impl ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

pub trait Space<K, V> {
    fn bounds(&self) -> (K, K);
}

impl<K, V> Space<K, V> for HashMap<K, V>
where
    K: Point + Ord,
    V: Default + Copy + Clone,
{
    fn bounds(&self) -> (K, K) {
        let mut min = self.keys().nth(0).unwrap();
        let mut max = min;

        for key in self.keys() {
            min = min.min(key);
            max = max.max(key);
        }

        (*min, *max)
    }
}

impl<K, V> Traversable<K> for HashMap<K, V>
where
    K: Point,
{
    type State = Self;

    fn is_traversable(&self, from: &K, to: &K) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct InnerType {
        value: bool,
    }

    struct Collection {
        inner: HashMap<Coordinate, InnerType>,
    }

    #[test]
    fn main_test() {
        let origin = Coordinate { x: 0, y: 0 };
        let max = Coordinate { x: 5, y: 5 };
        let mut space: HashMap<_, _> = origin
            .range(&max)
            .map(|c| (c, InnerType { value: true }))
            .collect();

        space.bounds();
    }
}
