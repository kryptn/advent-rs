use std::collections::HashSet;

use advent::{
    grid::{print_grid, Coordinate, Grid, RelativeDirection},
    input_store,
};

#[derive(Clone, Debug)]
struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Rope {
    fn from((head, tail): (Coordinate, Coordinate)) -> Self {
        Self { head, tail }
    }
}

impl Rope {
    fn new() -> Self {
        let head = (0, 0).into();
        let tail = (0, 0).into();
        Self { head, tail }
    }

    fn step(&self, direction: RelativeDirection) -> Self {
        // println!("\n\n\n");
        // dbg!(self, direction);

        let head = self.head;
        let tail = self.tail;

        let head = head + direction.into();
        let out = if head.neighbors().contains(&tail) || head == tail {
            (head, tail).into()
        } else {
            (head, head + direction.other().into()).into()
        };

        // dbg!(&out);

        out
    }
}

fn main() {
    let input = input_store::get_input(2022, 09);

    // let input = r#"R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2"#;

    let directions: Vec<RelativeDirection> = input
        .trim()
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.trim().split_whitespace().collect();
            let direction: RelativeDirection = split[0].into();
            let times = split[1].parse().unwrap();
            let directions: Vec<RelativeDirection> =
                (0..times).into_iter().map(|_| direction).collect();
            directions
        })
        .flatten()
        .collect();

    // dbg!(&directions);

    let mut rope = Rope::new();
    let mut states = vec![rope.clone()];

    for direction in directions {
        rope = rope.step(direction);
        states.push(rope.clone())
    }

    let visited: HashSet<Coordinate> = states.iter().map(|r| r.tail).collect();

    // let grid: Grid<i32> = visited.iter().map(|&c| (c, 1)).collect();
    // print_grid(&grid);

    println!("part_1 => {}", visited.len());
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
