use std::{rc::Rc, sync::Mutex};

use advent::{
    grid::{Coordinate, Grid},
    input_store,
};

struct Tile {
    height: usize,
    basin: Option<Rc<Mutex<Vec<Coordinate>>>>,
}

struct OceanFloor {
    floor: Grid<Tile>,
}

impl OceanFloor {
    fn spread(&self, target: &Coordinate) {
        let cardinals = [target.up(), target.right(), target.down(), target.left()];

        for coord in cardinals {
            if let Some(tile) = self.floor.get(&coord) {
                if tile.basin.is_none() {}
            }
        }
    }
}

fn main() {
    let input = input_store::get_input(2021, 09);
    //     let input = r#"2199943210
    // 3987894921
    // 9856789892
    // 8767896789
    // 9899965678"#;

    let mut floor: Grid<usize> = Grid::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            let height = height.to_string().parse().unwrap();

            floor.insert(Coordinate::new(x as i32, y as i32), height);
        }
    }

    let mut lowest: Vec<Coordinate> = Vec::new();

    for (coordinate, height) in floor.iter() {
        let cardinals = [
            coordinate.up(),
            coordinate.right(),
            coordinate.down(),
            coordinate.left(),
        ];

        if cardinals.iter().all(|c| match &floor.get(c) {
            Some(h) => &height < h,
            None => true,
        }) {
            lowest.push(coordinate.clone());
        }
    }

    let risk: usize = lowest
        .iter()
        .map(|c| {
            let height = floor.get(c).unwrap();
            height + 1
        })
        .sum();

    println!("part_1 => {}", risk);

    let mut ocean_floor: Grid<Tile> = Grid::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            let height = height.to_string().parse().unwrap();

            floor.insert(Coordinate::new(x as i32, y as i32), height);
        }
    }

    println!("part_2 => {}", "not done");
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
