use std::{collections::HashMap, hash::Hash, ops::Add};

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
    fn new() -> Self {
        Space(HashMap::new())
    }
}

// impl <P: Point, T: std::fmt::Display + Default> Space<P, T> {
//     pub fn print_grid(&self)

//     {
//     let (lower, upper) = bounding_box(&g);

//     for row in iter_rows(lower, upper) {
//         for coord in row {
//             let item = match g.get(&coord) {
//                 Some(i) => i.clone(),
//                 None => T::default(),
//             };
//             print!("{}", item);
//         }
//         print!("\n");
//     }
//     println!("");
// }
// }

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        ((self.x + rhs.x), (self.y + rhs.y)).into()
    }
}

macro_rules! coord_from {
    ( $x:ty ) => {
        impl From<($x, $x)> for Coordinate {
            fn from((x, y): ($x, $x)) -> Self {
                let x = x as i32;
                let y = y as i32;
                Self { x, y }
            }
        }
    };
}

coord_from!(usize);
coord_from!(isize);
coord_from!(u32);
coord_from!(i32);

impl Point for Coordinate {}

impl Point for (i32, i32) {}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_grid() {
        let objects = vec![((2, 4), false)];
        let g: Space<(i32, i32), bool> = objects.into_iter().collect();
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

        let c: Coordinate = (1 as i32, 1 as i32).into();
        assert_eq!(c, expected);
    }
}
