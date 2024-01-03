use advent::input_store;
use advent_toolbox::{
    algo::dijkstra,
    spatial::{self, Coordinate, Space},
};
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

fn find_path<const MIN: usize, const MAX: usize>(city: &Space<Coordinate, Block>) -> usize {
    let (lower, upper) = city.bounds();

    let edges = |n: &Node| -> Vec<Node> { n.next_steps(MIN, MAX) };

    let is_goal = |n: &Node| -> bool { n.position == upper };

    let cost_fn = |n: &Node| -> Option<usize> {
        match city.get(&n.position) {
            Some(b) => Some(b.thermal_mass),
            None => None,
        }
    };

    let initial_right = Node {
        position: lower,
        direction: spatial::Direction::Right,
        repeated: 0,
    };

    let initial_up = Node {
        position: lower,
        direction: spatial::Direction::Up,
        repeated: 0,
    };

    let path: Vec<Node> = dijkstra(&[initial_right, initial_up], edges, is_goal, Some(cost_fn))
        .into_iter()
        .collect();

    let heat_loss = path
        .iter()
        .skip(1)
        .map(|n| city.get(&n.position).unwrap().thermal_mass)
        .sum::<usize>();

    heat_loss
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

    let part_1 = find_path::<0, 3>(&city);
    println!("part_1 => {}", part_1);

    let part_2 = find_path::<4, 10>(&city);
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
