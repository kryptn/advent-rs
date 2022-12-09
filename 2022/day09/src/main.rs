use std::collections::HashSet;

use advent::{
    grid::{manhattan, Coordinate, RelativeDirection},
    input_store,
};
use itertools::Itertools;

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
        let head_neighbors: HashSet<_> = head.neighbors().iter().cloned().collect();
        let tail_neighbors: HashSet<_> = tail.neighbors().iter().cloned().collect();
        let nt = head_neighbors
            .intersection(&tail_neighbors)
            .sorted_by(|lhs, rhs| {
                let lhsm = manhattan(head, **lhs);
                let rhsm = manhattan(head, **rhs);
                lhsm.cmp(&rhsm)
            })
            .next()
            .unwrap();

        *nt
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
        let head = self.0[0];
        let head = head + direction.into();

        let mut next = vec![head];

        for knot in self.0[1..].iter() {
            let prev = next[next.len() - 1];
            next.push(tail_pos(prev, *knot))
        }

        Self(next)
    }

    fn tail(&self) -> Coordinate {
        self.0[self.0.len() - 1]
    }
}

fn visited_with_knots(knots: usize, directions: &Vec<RelativeDirection>) -> usize {
    let mut rope = Rope::new_with_knots(knots);
    let mut states = vec![rope.clone()];
    for direction in directions {
        rope = rope.step(*direction);
        states.push(rope.clone())
    }
    let visited: HashSet<Coordinate> = states.iter().map(|r| r.tail()).collect();
    visited.len()
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

    println!("part_1 => {}", visited_with_knots(2, &directions));
    println!("part_2 => {}", visited_with_knots(10, &directions));
}

#[cfg(test)]
mod test {

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
