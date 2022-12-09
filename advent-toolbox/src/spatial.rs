// use std::{collections::HashMap, hash::Hash};

// trait Point {}

// struct Space<P, T>(HashMap<P, T>)
// where
//     P: Point;

// impl<P: Point, T> std::ops::Deref for Space<P, T> {
//     type Target = HashMap<P, T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<P: Point, T> std::ops::DerefMut for Space<P, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl<P: Point, T> Space<P, T> {
//     fn new() -> Self {
//         Space(HashMap::new())
//     }
// }

// impl Point for (i32, i32) {}

// // impl<P, T> From<&dyn FromIterator<(P, T)>> for Space<P, T>
// // where
// //     P: Point + Eq + Hash,
// // {
// //     fn from(item: &mut dyn Iterator<Item = (P, T)>) -> Self {
// //         let inner = item.collect();
// //         Self(inner)
// //     }
// // }

// impl<'a, P: 'a + 'static, T: 'a + 'static> FromIterator<&'static dyn Iterator<Item = (P, T)>> for Space<P, T>
// where
//     P: Point,
// {
//     fn from_iter<O: IntoIterator<Item = &'static dyn Iterator<Item = (P, T)>>>(iter: O) -> Self {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod test {

//     use super::*;

//     #[test]
//     fn create_grid() {
//         let objects = vec![((1, 1), false)];
//         let g: Space<(i32, i32), bool> = objects.collect();
//     }
// }
