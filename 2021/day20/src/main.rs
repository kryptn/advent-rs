use std::collections::HashSet;

use advent::{
    grid::{bounding_box, iter_rows, print_grid, Coordinate, Grid},
    input_store,
};

#[derive(Debug)]
struct Image {
    mask: [bool; 512],
    points: Grid<PrintableBool>,
    iteration: usize,
}

#[derive(Debug, Default, Clone)]
struct PrintableBool(bool);

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = String::new();

        let (lower, upper) = bounding_box(&self.points);
        for row in iter_rows(lower, upper) {
            for coord in row {
                let item = match self.points.get(&coord) {
                    Some(i) => i.clone(),
                    None => PrintableBool::default(),
                };
                if item.0 == (self.iteration % 2 == 0) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }

        write!(f, "{}\niteration: {}", grid, self.iteration)
    }
}

// impl std::ops::Deref for PrintableBool {
//     type Target = bool;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl Image {
    fn advance(&mut self) {
        let mut out = Grid::new();

        let candidates: HashSet<Coordinate> = self
            .points
            .iter()
            .filter(|(_, v)| v.0)
            .map(|(k, _)| k.neighbors())
            .flatten()
            // .map(|c| c.neighbors())
            // .flatten()
            .collect();

        for candidate in candidates {
            let mut idx = 0;
            for neighbor in candidate.grid_around() {
                let value = {
                    if let Some(value) = self.points.get(&neighbor) {
                        value.0
                    } else {
                        self.iteration % 2 != 0
                    }
                };

                if value {
                    idx += 1;
                }

                idx <<= 1;
            }
            //dbg!(idx);
            idx >>= 1;

            let insertable = self.iteration % 2 != 0;

            if self.mask[idx] == insertable {
                out.insert(candidate, PrintableBool(!insertable));
            }
        }
        self.points = out;
        self.iteration += 1;
    }

    fn lit(&self) -> usize {
        self.points
            .iter()
            .filter(|(_, v)| v.0 == (self.iteration % 2 == 0))
            .count()
    }
}

impl From<String> for Image {
    fn from(input: String) -> Self {
        let mut input_lines = input.trim().lines();
        let lookup = input_lines.next().unwrap();
        let mut mask = [false; 512];

        for (i, c) in lookup.trim().chars().enumerate() {
            mask[i] = c == '#';
        }

        input_lines.next().unwrap();

        let mut points = Grid::new();
        for (y, line) in input_lines.enumerate() {
            for (x, state) in line.trim().chars().enumerate() {
                if state == '#' {
                    points.insert((x as i32, y as i32).into(), PrintableBool(true));
                }
            }
        }

        Self {
            mask,
            points,
            iteration: 0,
        }
    }
}

fn main() {
    let input = input_store::get_input(2021, 20);

    let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#.to_string();

    let mut image: Image = input.into();
    //dbg!(&image);

    println!("{}", image);
    image.advance();
    println!("{}", image);
    image.advance();
    println!("{}", image);

    println!("part_1 => {}", image.lit());
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
