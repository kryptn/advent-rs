use std::collections::HashMap;

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space, ORIGIN, UP};

const YEAR: usize = 2024;
const DAY: usize = 06;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Open,
    Closed,
    Agent(Agent),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Open => '.',
            Self::Closed => '#',
            Self::Agent(agent) => match agent.direction {
                Coordinate { x: 0, y: 1 } => 'v',
                Coordinate { x: 0, y: -1 } => '^',
                Coordinate { x: 1, y: 0 } => '>',
                Coordinate { x: -1, y: 0 } => '<',
                _ => panic!("Invalid direction"),
            },
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Agent {
    position: Coordinate,
    direction: Coordinate,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Closed,
            '^' => Self::Agent(Agent {
                position: ORIGIN,
                direction: UP,
            }),
            _ => panic!("Invalid tile"),
        }
    }
}

type Lab = Space<Coordinate, Tile>;

fn step(lab: &Lab, agent: &Agent) -> Option<Agent> {
    let next_step = agent.position + agent.direction;
    if let Some(next_tile) = lab.get(&next_step) {
        // dbg!(&agent);

        return match next_tile {
            Tile::Open => Some(Agent {
                position: next_step,
                direction: agent.direction,
            }),
            Tile::Closed => Some(Agent {
                position: agent.position,
                direction: agent.direction.turn_right(),
            }),
            _ => panic!("Invalid tile"),
        };
    }

    // dbg!(&agent);

    None
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    //     let input = r#"....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...
    // "#;

    let mut lab: Lab = Space::from_lines_rev(&input);
    let guard = lab.find(|t| matches!(t, Tile::Agent(_))).unwrap();
    *lab.get_mut(&guard).unwrap() = Tile::Open;

    let mut agent = Agent {
        position: guard,
        direction: Coordinate::new(0, 1),
    };

    let mut visited: HashMap<Coordinate, usize> = HashMap::new();
    *visited.entry(agent.position).or_insert(0) += 1;

    while let Some(next_agent) = step(&lab, &agent) {
        // let mut lab_p = lab.clone();
        // *lab_p.get_mut(&agent.position).unwrap() = Tile::Agent(agent);
        // println!("{}", lab_p);

        agent = next_agent;
        *visited.entry(agent.position).or_insert(0) += 1;
    }

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
