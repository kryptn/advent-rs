use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    io::{self, Write},
    pin::Pin,
    str::FromStr,
};

use advent::{
    fetch,
    grid::{self, Coordinate},
};
use anyhow;
use itertools::Itertools;

// fn step(grid: &grid::Grid<i32>) -> Vec<grid::Coordinate> {
//     let mut out: Vec<grid::Coordinate> = Vec::new();

//     let candidates: HashSet<grid::Coordinate>

//     out
// }

fn show_grid(grid: &grid::Grid<i32>) {
    let (lower, upper) = grid::bounding_box(grid);

    let mut out: String = String::from("");

    for y in lower.y..=upper.y {
        for x in lower.x..=upper.x {
            if grid.get(&grid::Coordinate::new(x, y)).unwrap_or(&0) == &1 {
                out.push_str("#");
            } else {
                out.push_str(".");
            }
        }
        out.push_str("\n");
    }
    out.push_str("\n\n");
    print!("{}", out);
}

fn main() {
    let input = fetch::get_input(2015, 18);

    //     let input = r#".#.#.#
    // ...##.
    // #....#
    // ..#...
    // #.#..#
    // ####.."#;

    let mut grid = grid::Grid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.trim().chars().enumerate() {
            let alive = {
                if cell == '#' {
                    1
                } else {
                    0
                }
            };

            let coordinate = grid::Coordinate::new(x as i32, y as i32);
            grid.insert(coordinate, alive);
        }
    }

    let bounding_box = grid::bounding_box(&grid);
    let other_corners = (
        grid::Coordinate::new(bounding_box.1.x, 0),
        Coordinate::new(0, bounding_box.1.y),
    );
    let bounding_box = vec![
        bounding_box.0,
        bounding_box.1,
        other_corners.0,
        other_corners.1,
    ];

    for coord in bounding_box.iter() {
        grid.insert(*coord, 1);
    }

    //dbg!(input);
    //dbg!(grids);

    for _ in 0..100 {
        show_grid(&grid);
        let mut next_grid = grid::Grid::<i32>::new();
        for (coordinate, &state) in grid.iter() {
            //println!("checking neighbors for {:?}", coordinate);
            let neighbors: i32 = coordinate
                .neighbors()
                .iter()
                .map(|c| {
                    match grid.get(c) {
                        Some(v) => {
                            //println!("{:?} -> got {}", &c, v);
                            v
                        }
                        None => {
                            //println!("{:?} -> default, defaulting to 0", &c);
                            &0
                        }
                    }

                    //let neighbor = grid.get(c).unwrap_or(&0);
                })
                .cloned()
                .sum();
            let next_state = {
                if bounding_box.contains(coordinate) {
                    1
                } else if neighbors == 3 || (neighbors == 2 && state == 1) {
                    1
                } else {
                    0
                }
            };
            next_grid.insert(*coordinate, next_state);
        }
        grid = next_grid;
    }

    show_grid(&grid);

    println!(
        "part 1 -> {:?}",
        grid.iter()
            .filter(|(key, value)| *value == &1)
            .collect_vec()
            .len()
    )
}

#[cfg(test)]
mod test {

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
