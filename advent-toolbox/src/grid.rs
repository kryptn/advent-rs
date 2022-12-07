// use std::{ops::Add, collections::HashMap};

// trait Point {
//     type Coordinate;
// }

// // trait Space {}

// // struct Coordinate {}

// struct Space(HashMap<T, V>);

// #[cfg(test)]
// mod test {
//     use super::*;

//     struct Cell {
//         active: bool,
//     }

//     impl From<char> for Cell {
//         fn from(c: char) -> Self {
//             match c {
//                 '.' => Self { active: false },
//                 '#' => Self { active: true },
//                 _ => panic!(),
//             }
//         }
//     }


//     struct Coordinate {
//         x: i32,
//         y: i32,
//     }

//     impl Point for Cell {
//         type Coordinate = Coordinate;
//     }

//     const example: &str = r#"
// .....
// ..#..
// ...#.
// .###.
// .....
// "#;
// }
