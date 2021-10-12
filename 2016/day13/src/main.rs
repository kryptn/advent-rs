use std::fmt::Display;

use advent::{
    grid::{iter_rows, print_grid, traverse_astar, Coordinate, Grid, Passable, Traversal},
    input_store,
};

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

    if let Traversal::Found(p) = path {
        for coord in grid.keys() {
            let tc = TraversedCell {
                coordinate: coord.clone(),
                traversed: p.contains(coord),
            };
            out.insert(coord.clone(), tc);
        }
    } else {
        panic!();
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
    let input = input_store::get_input(2016, 13);

    let grid = generate_map(40, 40);

    let start = Coordinate::new(1, 1);
    let goal = Coordinate::new(31, 39);
    //let goal = Coordinate::new(7, 4);

    print_grid(&grid);

    let path = traverse_astar::<Cell>(&grid, start, goal);
    //dbg!(&path);

    let ng = generate_traversed_map(&grid, path.clone());
    print_grid(&ng);

    match path {
        advent::grid::Traversal::Found(p) => println!("part 1 => {}", p.len() - 1),
        advent::grid::Traversal::NoPath => println!("uhhh"),
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
