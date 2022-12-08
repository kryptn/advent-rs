use std::collections::HashMap;

use advent::{
    grid::{print_grid, Coordinate, Grid, RelativeDirection},
    input_store,
};

struct Forest {
    trees: Grid<i32>,
    visible: Grid<bool>,

    max_x: i32,
    max_y: i32,
}

impl Forest {
    fn leftmost(&self) -> Vec<Coordinate> {
        let mut out = Vec::new();
        for y in 0..=self.max_y {
            out.push((0, y).into())
        }
        out
    }

    fn rightmost(&self) -> Vec<Coordinate> {
        let mut out = Vec::new();
        for y in 0..=self.max_y {
            out.push((self.max_x, y).into())
        }
        out
    }

    fn bottommost(&self) -> Vec<Coordinate> {
        let mut out = Vec::new();
        for x in 0..=self.max_x {
            out.push((x, 0).into())
        }
        out
    }

    fn topmost(&self) -> Vec<Coordinate> {
        let mut out = Vec::new();
        for x in 0..=self.max_x {
            out.push((x, self.max_y).into())
        }
        out
    }

    fn line_from(&self, start: Coordinate, dir: RelativeDirection) -> Vec<Coordinate> {
        let mut out = Vec::new();
        let mut coord = start;
        let delta = dir.into();
        while self.trees.contains_key(&coord) {
            out.push(coord);
            coord = coord + delta;
        }

        out
    }

    fn edges(&self) -> Vec<(Coordinate, RelativeDirection)> {
        let leftmost = self.leftmost();
        let left_edge = leftmost.iter().map(|&c| (c, RelativeDirection::Right));
        let rightmost = self.rightmost();
        let right_edge = rightmost.iter().map(|&c| (c, RelativeDirection::Left));
        let topmost = self.topmost();
        let top_edge = topmost.iter().map(|&c| (c, RelativeDirection::Down));
        let bottommost = self.bottommost();
        let bottom_edge = bottommost.iter().map(|&c| (c, RelativeDirection::Up));

        let edges: Vec<(Coordinate, RelativeDirection)> = left_edge
            .chain(right_edge)
            .chain(top_edge)
            .chain(bottom_edge)
            .collect();

        edges
    }

    fn mark_visible_trees(&mut self) {
        for (start, dir) in self.edges() {
            let mut max_height = -1;
            for coord in self.line_from(start.clone(), dir) {
                let tree_height = self.trees.get(&coord).unwrap();
                if tree_height > &max_height {
                    max_height = *tree_height;
                    self.visible.insert(coord, true);
                }
            }
        }
    }

    fn visible_from(&self, from: Coordinate, dir: RelativeDirection) -> u32 {
        let this_height = self.trees.get(&from).unwrap();
        let mut visible = 0;
        for coord in self.line_from(from + dir.into(), dir) {
            let tree_height = self.trees.get(&coord).unwrap();
            visible += 1;

            if tree_height >= this_height {
                break;
            }
        }

        visible
    }

    fn scenic_score(&self, from: Coordinate) -> u32 {
        let left = self.visible_from(from, RelativeDirection::Left);
        let right = self.visible_from(from, RelativeDirection::Right);
        let up = self.visible_from(from, RelativeDirection::Up);
        let down = self.visible_from(from, RelativeDirection::Down);

        left * right * up * down
    }
}

impl From<HashMap<Coordinate, i32>> for Forest {
    fn from(trees: HashMap<Coordinate, i32>) -> Self {
        let visible = HashMap::new();
        let max_x = trees.keys().map(|coord| coord.x).max().unwrap();
        let max_y = trees.keys().map(|coord| coord.y).max().unwrap();

        Self {
            trees: trees.clone(),
            visible,
            max_x: max_x.clone(),
            max_y: max_y.clone(),
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 08);
    // let input = r#"30373
    // 25512
    // 65332
    // 33549
    // 35390"#;

    let mut grid = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let coordinate: Coordinate = (x, y).into();
            let d = c.to_digit(10).unwrap();
            grid.insert(coordinate, d as i32);
        }
    }

    let mut forest: Forest = grid.into();
    forest.mark_visible_trees();

    // dbg!(&forest.visible);

    // print_grid(&forest.trees);
    // print_grid(&forest.visible);

    // dbg!(forest.edges());
    // dbg!(forest.rightmost());

    // for (edge, dir) in forest.edges() {
    //     print!("start: {}, going: {:?}", edge, dir);
    //     for coord in forest.line_from(edge, dir) {
    //         print!("{} ", coord)
    //     }
    //     println!("")
    // }

    println!("part_1 => {}", forest.visible.len());

    let scenics: Grid<u32> = forest
        .trees
        .keys()
        .map(|&c| (c, forest.scenic_score(c)))
        .collect();
    // print_grid(&scenics);

    println!("part_2 => {}", scenics.values().max().unwrap());
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
