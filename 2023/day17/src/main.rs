use std::{cmp::Reverse, collections::HashMap};

use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Space};
use colored::Colorize;

const YEAR: usize = 2023;
const DAY: usize = 17;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Block {
    thermal_mass: usize,
    visited: bool,
}

impl From<char> for Block {
    fn from(c: char) -> Self {
        let thermal_mass = c.to_digit(10).unwrap() as usize;
        Self {
            thermal_mass,
            visited: false,
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visited {
            write!(f, "{}", self.thermal_mass.to_string().green())
        } else {
            write!(f, "{}", self.thermal_mass.to_string())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Node {
    // score: usize,
    position: Coordinate,
    direction: spatial::Direction,
    repeated: usize,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
        self.repeated.hash(state);
    }
}

impl Node {
    fn next_steps(&self, min_steps: usize, max_steps: usize) -> Vec<Self> {
        let mut out = vec![];

        if self.repeated < min_steps {
            return vec![Self {
                position: self.position + self.direction,
                direction: self.direction,
                repeated: self.repeated + 1,
            }];
        }

        let next_directions = &[
            self.direction,
            self.direction.left(),
            self.direction.right(),
        ];
        for direction in next_directions {
            let position = self.position + *direction;
            let repeated = if *direction == self.direction {
                self.repeated + 1
            } else {
                1
            };

            if repeated > max_steps {
                continue;
            }

            out.push(Self {
                // score: self.score,
                position,
                direction: *direction,
                repeated,
            });
        }

        out
    }
}

fn dijkstra(
    start: &Coordinate,
    goal: &Coordinate,
    cost_fn: impl Fn(&Coordinate) -> Option<usize>,
    min_steps: usize,
    max_steps: usize,
) -> (usize, Vec<Coordinate>) {
    let mut queue = std::collections::BinaryHeap::new();

    let mut distances = HashMap::new();
    let mut prev_map = HashMap::new();

    let initial_right = Node {
        position: *start,
        direction: spatial::Direction::Right,
        repeated: 0,
    };

    let initial_up = Node {
        position: *start + spatial::Direction::Up,
        direction: spatial::Direction::Up,
        repeated: 0,
    };

    distances.insert(initial_right, 0);
    distances.insert(initial_up, 0);

    queue.push(Reverse(initial_right));
    queue.push(Reverse(initial_up));

    while !queue.is_empty() {
        let Reverse(node) = queue.pop().unwrap();
        for next in node.next_steps(min_steps, max_steps) {
            if let Some(cost) = cost_fn(&next.position) {
                let next_dist = distances.get(&node).unwrap() + cost;

                if &next_dist < distances.get(&next).unwrap_or(&usize::MAX) {
                    distances.insert(next, next_dist);
                    prev_map.insert(next, node);
                    queue.push(Reverse(next));
                }
            }
        }
    }

    let (node, goal_distance) = distances
        .iter()
        .filter(|(k, _)| k.position == *goal && k.repeated >= min_steps)
        .min_by_key(|v| v.1)
        .unwrap();

    let path = std::iter::successors(Some(*node), |n| prev_map.get(n).copied())
        .map(|n| n.position)
        .collect();

    let goal_cost = cost_fn(goal).unwrap();

    (*goal_distance + goal_cost, path)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let input = input.as_str();
    // let input = r#"2413432311323
    // 3215453535623
    // 3255245654254
    // 3446585845452
    // 4546657867536
    // 1438598798454
    // 4457876987766
    // 3637877979653
    // 4654967986887
    // 4564679986453
    // 1224686865563
    // 2546548887735
    // 4322674655533"#;

    // let input = r#"111111111111
    // 999999999991
    // 999999999991
    // 999999999991
    // 999999999991"#;

    let city = Space::<Coordinate, Block>::from(input);

    let (lower, upper) = city.bounding_box();

    let min_steps = 0;
    let max_steps = 3;

    let (part_1, path) = dijkstra(
        &lower,
        &upper,
        |c| match city.get(&c) {
            Some(b) => Some(b.thermal_mass),
            None => None,
        },
        min_steps,
        max_steps,
    );

    // let mut blocked_city: Space<_, _> = city.clone().into_iter().collect();
    // for c in path {
    //     blocked_city.get_mut(&c).unwrap().visited = true;
    // }
    // println!("{}\n", blocked_city);

    println!("part_1 => {}", part_1);

    let min_steps = 4;
    let max_steps = 10;
    let (part_2, path) = dijkstra(
        &lower,
        &upper,
        |c| match city.get(&c) {
            Some(b) => Some(b.thermal_mass),
            None => None,
        },
        min_steps,
        max_steps,
    );

    // let mut blocked_city: Space<_, _> = city.clone().into_iter().collect();
    // for c in path {
    //     blocked_city.get_mut(&c).unwrap().visited = true;
    // }
    // println!("{}\n", blocked_city);

    println!("part_2 => {}", part_2);
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
