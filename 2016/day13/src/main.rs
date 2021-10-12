use std::{collections::HashSet, fmt::Display};

use advent::{
    grid::{iter_rows, print_grid, traverse_astar, Coordinate, Grid, Passable, Traversal},
    input_store,
};
use itertools::Itertools;

const FAV: i32 = 1362;
//const FAV: i32 = 10;

struct Cell {
    coordinate: Coordinate,
}

impl Passable for Cell {
    fn is_passable(&self) -> bool {
        let x = self.coordinate.x;
        let y = self.coordinate.y;

        let v = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + FAV;
        v.count_ones() % 2 == 0
    }
}

impl From<Coordinate> for Cell {
    fn from(coordinate: Coordinate) -> Self {
        Self { coordinate }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_passable() { "." } else { "#" })
    }
}

struct TraversedCell {
    coordinate: Coordinate,
    traversed: bool,
}

impl Passable for TraversedCell {
    fn is_passable(&self) -> bool {
        let x = self.coordinate.x;
        let y = self.coordinate.y;

        let v = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + FAV;
        v.count_ones() % 2 == 0
    }
}

impl Display for TraversedCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = if self.is_passable() {
            if self.traversed {
                "O"
            } else {
                "."
            }
        } else {
            if self.traversed {
                "?"
            } else {
                "#"
            }
        };
        write!(f, "{}", v)
    }
}

fn generate_traversed_map(grid: &Grid<Cell>, path: Traversal) -> Grid<TraversedCell> {
    let mut out = Grid::new();

    match path {
        Traversal::Found(p) => {
            for coord in grid.keys() {
                let tc = TraversedCell {
                    coordinate: coord.clone(),
                    traversed: p.contains(coord),
                };
                out.insert(coord.clone(), tc);
            }
        }
        Traversal::NoPath(p) => {
            for coord in grid.keys() {
                let tc = TraversedCell {
                    coordinate: coord.clone(),
                    traversed: p.contains(coord),
                };
                out.insert(coord.clone(), tc);
            }
        }
    }

    out
}

fn generate_map(x: i32, y: i32) -> Grid<Cell> {
    let lower = Coordinate::zero();
    let upper = Coordinate::new(x + 5, y + 5);

    let mut grid = Grid::new();

    for row in iter_rows(lower, upper) {
        for coord in row {
            let cell: Cell = coord.into();
            grid.insert(coord, cell);
        }
    }

    grid
}

fn main() {
    //let input = input_store::get_input(2016, 13);

    let grid = generate_map(40, 40);

    let start = Coordinate::new(1, 1);
    let goal = Coordinate::new(31, 39);

    //print_grid(&grid);

    let paths = traverse_astar::<Cell>(&grid, start, goal);
    let sorted_paths: Vec<Vec<Coordinate>> = paths
        .iter()
        .filter(|&p| matches!(p, Traversal::Found(_)))
        .map(|p| match p {
            Traversal::Found(fp) => fp.clone(),
            Traversal::NoPath(_) => unreachable!(),
        })
        .sorted_by(|l, r| l.len().cmp(&r.len()))
        .collect();
    let shortest = sorted_paths.iter().nth(0).unwrap().clone();

    // let ng = generate_traversed_map(&grid, Traversal::Found(shortest.clone()));
    // print_grid(&ng);
    println!("part 1 => {}", shortest.len() - 1);

    let mut visited: HashSet<Coordinate> = HashSet::new();

    let mut insert = |path: Vec<Coordinate>| {
        for c in path.iter().take(51) {
            visited.insert(c.clone());
        }
    };

    for path in paths {
        match path {
            Traversal::Found(p) => insert(p),
            Traversal::NoPath(p) => insert(p),
        }
    }
    let visited: Vec<Coordinate> = visited.iter().cloned().collect();
    println!("part 2 => {}", visited.len());

    // let ng = generate_traversed_map(&grid, Traversal::NoPath(visited));
    // print_grid(&ng);
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
