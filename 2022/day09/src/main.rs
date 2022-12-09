use std::collections::HashSet;

use advent::{
    grid::{print_grid, Coordinate, Grid, RelativeDirection},
    input_store,
};

#[derive(Clone, Debug)]
struct Rope(Vec<Coordinate>);

impl From<(Coordinate, Coordinate)> for Rope {
    fn from((head, tail): (Coordinate, Coordinate)) -> Self {
        Self(vec![head, tail])
    }
}

fn tail_pos(head: Coordinate, tail: Coordinate) -> Coordinate {
    if head == tail || head.neighbors().contains(&tail) {
        tail
    } else {
        let head_ortho_neighbors: HashSet<_> = head.ortho_neighbors().iter().cloned().collect();
        let tail_neighbors: HashSet<_> = tail.neighbors().iter().cloned().collect();
        // dbg!(&head, &tail);
        // dbg!(&head_neighbors, &tail_neighbors);
        let itx = match head_ortho_neighbors.intersection(&tail_neighbors).next() {
            Some(itx) => itx.clone(),
            None => {
                let head_neighbors: HashSet<_> = head.neighbors().iter().cloned().collect();
                head_neighbors
                    .intersection(&tail_neighbors)
                    .next()
                    .unwrap()
                    .clone()
            }
        };
        // dbg!(&itx);
        // println!("\n\n\n");

        itx
    }
}

impl Rope {
    fn new_with_knots(knots: usize) -> Self {
        let mut rope = Vec::new();
        for _ in 0..knots {
            rope.push((0, 0).into());
        }

        Self(rope)
    }

    fn step(&self, direction: RelativeDirection) -> Self {
        // println!("\n\n\n");
        // dbg!(self, direction);

        let head = self.0[0];
        let head = head + direction.into();

        // println!("moved head from {} to {}", self.0[0], head);

        let mut next = vec![head];

        for (idx, knot) in self.0[1..].iter().enumerate() {
            let prev = next[next.len() - 1];
            // println!("comparing idx {}\nhead: {:?}\ntail:{:?}\n", idx+1, prev, knot);
            next.push(tail_pos(prev, *knot))
        }

        // dbg!(&out);

        Self(next)
    }

    fn tail(&self) -> Coordinate {
        self.0[self.0.len() - 1]
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
    // let input = r#"R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20"#;

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

    // let mut rope = Rope(vec![(0, 0).into(), (0, 0).into()]);

    let mut rope = Rope::new_with_knots(2);
    let mut states = vec![rope.clone()];
    for direction in &directions {
        rope = rope.step(*direction);
        states.push(rope.clone())
    }
    let visited: HashSet<Coordinate> = states.iter().map(|r| r.tail()).collect();

    // let grid: Grid<i32> = visited.iter().map(|&c| (c, 1)).collect();
    // print_grid(&grid);

    println!("part_1 => {}", visited.len());

    let mut rope = Rope::new_with_knots(10);
    let mut states = vec![rope.clone()];
    for direction in &directions {
        // println!("\n\n\n\nmoving {:?} --------------------------------\n", direction);
        // let g: Grid<usize> = rope.0.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        // print_grid(&g);
        rope = rope.step(*direction);
        states.push(rope.clone())
    }
    let visited: HashSet<Coordinate> = states.iter().map(|r| r.tail()).collect();
    println!("part_2 => {}", visited.len());
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
