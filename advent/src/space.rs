use std::{collections::HashMap, ops::Add};

// pub trait Space<T: Point> {
//     fn bounding_box(&self) -> (T, T);
// }

pub trait Point: Eq + PartialEq + Clone + Add {
    fn distance(&self, other: &Self) -> i32;
    fn range_to(&self, other: &Self) -> Vec<Self>;

    fn cardinals(&self) -> Vec<Self>;
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Coordinate2d {
    x: i32,
    y: i32,
}

const UP: Coordinate2d = Coordinate2d { x: 0, y: 1 };
const RIGHT: Coordinate2d = Coordinate2d { x: 0, y: 1 };
const DOWN: Coordinate2d = Coordinate2d { x: 0, y: 1 };
const LEFT: Coordinate2d = Coordinate2d { x: 0, y: 1 };

impl Coordinate2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Coordinate2d> for Coordinate2d {
    type Output = Coordinate2d;

    fn add(self, rhs: Coordinate2d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coordinate2d> for Coordinate2d {
    type Output = Coordinate2d;

    fn add(self, rhs: &Coordinate2d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn sorted<T: PartialEq + PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

impl Point for Coordinate2d {
    fn distance(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn range_to(&self, other: &Self) -> Vec<Self> {
        let mut coords: Vec<Coordinate2d> = Vec::new();

        let (y_left, y_right) = sorted(self.y, other.y);
        let (x_left, x_right) = sorted(self.x, other.x);

        for y in y_left..=y_right {
            for x in x_left..=x_right {
                coords.push(Coordinate2d::new(x, y))
            }
        }

        coords
    }

    fn cardinals(&self) -> Vec<Self> {
        vec![UP + self, RIGHT + self, DOWN + self, LEFT + self]
    }
}

#[derive(Clone)]
pub struct Path<T: Point> {
    pub path: Vec<T>,
}

impl<T> Add<T> for Path<T>
where
    T: Point,
{
    type Output = Path<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut out = self.path.clone();
        out.push(rhs);
        Self::Output { path: out }
    }
}

pub enum FinalPath<T: Point> {
    Found(Vec<T>),
    Ended(Vec<T>),
}

pub trait Traversal<T: Point> {
    fn next_steps(&self, visited: &Path<T>) -> Vec<T>;
    fn at_goal(&self, visited: &Path<T>) -> bool;

    fn traverse(&self, ctx: Path<T>) -> Vec<FinalPath<T>> {
        if self.at_goal(&ctx) {
            return vec![FinalPath::Found(ctx.path.clone())];
        }

        let directions = self.next_steps(&ctx);

        if directions.is_empty() {
            return vec![];
        }

        let results: Vec<FinalPath<T>> = directions
            .iter()
            .cloned()
            .map(|d| {
                let ctx = ctx.clone() + d;
                self.traverse(ctx)
            })
            .flatten()
            .collect();

        if results.is_empty() {
            return vec![FinalPath::Ended(ctx.path)];
        }

        results
    }
}

pub trait Passable<T>
where
    T: Point,
{
    fn is_passable(&self) -> bool;
}
