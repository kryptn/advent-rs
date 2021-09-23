use std::slice::Iter;

use advent::{fetch, grid::Coordinate, numbers::factors};
use itertools::{Combinations, Itertools, Powerset};

struct Paper {
    current: Option<Coordinate>,
}

impl Paper {
    fn new() -> Self {
        Self { current: None }
    }
}

impl Iterator for Paper {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == None {
            self.current = Some(Coordinate::new(1, 1));
        } else {
            let current = self.current.unwrap();
            let mut x = current.x + 1;
            let mut y = current.y - 1;
            if y < 1 {
                x = 1;
                y = current.x + 1;
            }
            self.current = Some(Coordinate { x, y })
        }

        self.current
    }
}

fn main() {
    // let input = fetch::get_input(2015, 20);

    let expected = Coordinate::new(3019, 3010);
    let mut paper = Paper::new();
    let last_coord = paper.next();
    let mut last_val: u64 = 20151125;

    for coord in paper {
        last_val = (last_val * 252533) % 33554393;
        if coord == expected {
            break;
        }
    }

    dbg!(last_coord, last_val);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_coordinate_iter() {
        let mut paper = Paper::new();

        assert_eq!(paper.next(), Some(Coordinate::new(1, 1)));
        assert_eq!(paper.next(), Some(Coordinate::new(1, 2)));
        assert_eq!(paper.next(), Some(Coordinate::new(2, 1)));
        assert_eq!(paper.next(), Some(Coordinate::new(1, 3)));
        assert_eq!(paper.next(), Some(Coordinate::new(2, 2)));
        assert_eq!(paper.next(), Some(Coordinate::new(3, 1)));
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
