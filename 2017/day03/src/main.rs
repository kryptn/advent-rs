use std::collections::HashMap;

use advent::input_store;

#[derive(Debug)]
struct Cursor {
    coord: (isize, isize),
    direction: (isize, isize),
}

impl Cursor {
    fn new() -> Self {
        Self {
            coord: (0, 0),
            direction: (1, 0),
        }
    }

    fn left(&self) -> (isize, isize) {
        let next = match self.direction {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => (0, 0),
        };
        next
    }

    fn coord_to_left(&self) -> (isize, isize) {
        let left = self.left();
        (self.coord.0 + left.0, self.coord.1 + left.1)
    }

    fn turn(&mut self) {
        self.direction = self.left();
    }

    fn forward(&self) -> (isize, isize) {
        (
            self.coord.0 + self.direction.0,
            self.coord.1 + self.direction.1,
        )
    }

    fn step(&mut self) {
        self.coord = self.forward();
    }
}

fn neighbors(coord: (isize, isize)) -> [(isize, isize); 8] {
    [
        (coord.0 - 1, coord.1 - 1),
        (coord.0, coord.1 - 1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1 + 1),
    ]
}

#[derive(Debug)]
enum Determinator {
    Index,
    NeighborSum,
}

#[derive(Debug)]
struct Ulam {
    coord_num: HashMap<(isize, isize), usize>,
    num_coord: HashMap<usize, (isize, isize)>,

    cursor: Cursor,
    determinator: Determinator,
}

impl Ulam {
    fn new(determinator: Determinator) -> Self {
        let coord_num = HashMap::new();
        let num_coord = HashMap::new();

        let cursor = Cursor::new();

        let mut this = Self {
            coord_num,
            num_coord,
            cursor,
            determinator,
        };

        this.insert_index((0, 0), 1);

        this
    }

    fn insert_index(&mut self, coord: (isize, isize), index: usize) {
        self.coord_num.insert(coord, index);
        self.num_coord.insert(index, coord);
    }

    fn next_value(&self) -> usize {
        match self.determinator {
            Determinator::Index => self.coord_num.get(&self.cursor.coord).unwrap() + 1,
            Determinator::NeighborSum => {
                let pos = self.cursor.forward();
                neighbors(pos)
                    .iter()
                    .map(|nc| self.coord_num.get(nc).unwrap_or(&0))
                    .sum::<usize>()
            }
        }
    }
}

impl Iterator for Ulam {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.next_value();
        self.cursor.step();
        self.insert_index(self.cursor.coord, next_value);

        if !self.coord_num.contains_key(&self.cursor.coord_to_left()) {
            self.cursor.turn();
        }

        Some(next_value)
    }
}

fn main() {
    let input = input_store::get_input(2017, 03)
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut ulam = Ulam::new(Determinator::Index);
    for _ in 0..input + 1 {
        ulam.next().unwrap();
    }

    let part_1_coord = ulam.num_coord.get(&input).unwrap();
    let part_1 = part_1_coord.0.abs() + part_1_coord.1.abs();
    println!("part_1 => {}", part_1);

    let mut ulam = Ulam::new(Determinator::NeighborSum);

    loop {
        let value = ulam.next().unwrap();
        if value > input {
            println!("part_2 => {}", value);
            break;
        }
    }
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
