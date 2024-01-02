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
    repeated: isize,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
    }
}

impl Node {
    fn next_steps(&self) -> Vec<Self> {
        let mut out = vec![];

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

            if repeated > 3 {
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
) -> usize {
    let mut queue = std::collections::BinaryHeap::new();

    let mut distances = HashMap::new();
    let mut prev_map = HashMap::new();

    let initial_down = Node {
        // score: 0,
        position: *start,
        direction: spatial::Direction::Down,
        repeated: 0,
    };

    let initial_left = Node {
        // score: 0,
        position: *start,
        direction: spatial::Direction::Left,
        repeated: 0,
    };

    distances.insert(initial_down, 0);
    distances.insert(initial_left, 0);

    queue.push(Reverse(initial_down));
    queue.push(Reverse(initial_left));

    while !queue.is_empty() {
        let Reverse(node) = queue.pop().unwrap();

        // if node.position == *goal {
        //     return distances.get(&node).unwrap().clone();
        // }

        for next in node.next_steps() {
            // if distances.contains_key(&next) {
            //     continue;
            // }

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

    let goal_distance = distances
        .iter()
        .filter(|(k, _)| k.position == *goal)
        .map(|(_, v)| v)
        .min()
        .unwrap();

    *goal_distance
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

    let city = Space::<Coordinate, Block>::from(input);

    let (lower, upper) = city.bounding_box();

    let part_1 = dijkstra(&lower, &upper, |c| match city.get(&c) {
        Some(b) => Some(b.thermal_mass),
        None => None,
    });

    println!("part_1 => {}", part_1);
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
