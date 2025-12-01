use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

use advent::input_store;
use advent_toolbox::spatial::{Agent, Coordinate, Space, DOWN, ORIGIN, UP};
use rayon::prelude::*;

const YEAR: usize = 2024;
const DAY: usize = 06;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Open,
    Closed,
    PotentiallyClosed,
    Agent(Agent),
    Visited(HashSet<Agent>),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Open => '.',
            Self::Closed => '#',
            Self::PotentiallyClosed => 'O',
            Self::Visited(agents) => {
                let mut vertical = false;
                let mut horizontal = false;

                for agent in agents {
                    if agent.direction == UP || agent.direction == DOWN {
                        vertical = true;
                    } else {
                        horizontal = true;
                    }
                }

                match (vertical, horizontal) {
                    (true, true) => '+',
                    (true, false) => '|',
                    (false, true) => '-',
                    _ => panic!("Invalid direction"),
                }
            }
            Self::Agent(agent) => match agent.direction {
                Coordinate { x: 0, y: 1 } => '^',
                Coordinate { x: 0, y: -1 } => 'v',
                Coordinate { x: 1, y: 0 } => '>',
                Coordinate { x: -1, y: 0 } => '<',
                _ => panic!("Invalid direction"),
            },
        };
        write!(f, "{}", c)
    }
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
            Tile::Closed | Tile::PotentiallyClosed => Some(Agent {
                position: agent.position,
                direction: agent.direction.turn_right(),
            }),
            _ => panic!("Invalid tile"),
        };
    }

    // dbg!(&agent);

    None
}

fn print_debug_lab(lab: &Lab, agent: &Agent, agents: &HashMap<Coordinate, HashSet<Agent>>) {
    let mut debug_lab = lab.clone();

    for visited_tile in agents.keys() {
        *debug_lab.get_mut(visited_tile).unwrap() = Tile::Visited(agents[visited_tile].clone());
    }
    *debug_lab.get_mut(&agent.position).unwrap() = Tile::Agent(agent.clone());
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\n\n{:?}\n\n", debug_lab);
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

    let initial_agent = Agent {
        position: guard,
        direction: UP,
    };

    let mut agent = initial_agent.clone();

    let mut visited: HashMap<Coordinate, HashSet<Agent>> = HashMap::new();
    visited
        .entry(agent.position)
        .or_insert(HashSet::new())
        .insert(agent.clone());

    let mut path = vec![agent.clone()];

    while let Some(next_agent) = step(&lab, &agent) {
        // let mut lab_p = lab.clone();
        // *lab_p.get_mut(&agent.position).unwrap() = Tile::Agent(agent);
        // println!("{}", lab_p);

        agent = next_agent;
        visited
            .entry(agent.position)
            .or_insert(HashSet::new())
            .insert(agent.clone());
        path.push(agent);
    }

    println!("part_1 => {}", visited.len());

    let mut loop_potentials = HashSet::new();
    loop_potentials.insert(initial_agent.position);

    loop_potentials.extend(
        path.par_iter()
            .filter_map(|agent| {
                let mut this_agent = agent.clone();

                let mut is_loop = false;
                let next_position = this_agent.position + this_agent.direction;

                if !loop_potentials.contains(&next_position) {
                    this_agent = initial_agent.clone();
                    if let Some(Tile::Open) = lab.get(&next_position) {
                        let mut this_visited: HashMap<Coordinate, HashSet<Agent>> = HashMap::new();
                        let mut this_lab = lab.clone();
                        this_lab.insert(next_position, Tile::PotentiallyClosed);

                        while let Some(next_agent) = step(&this_lab, &this_agent) {
                            this_agent = next_agent;
                            if let Some(agents) = this_visited.get(&this_agent.position) {
                                if agents.contains(&this_agent) {
                                    is_loop = true;
                                    // print_debug_lab(&this_lab, &this_agent, &this_visited);
                                    // sleep(Duration::from_millis(100));
                                    break;
                                }
                            }
                            this_visited
                                .entry(this_agent.position)
                                .or_insert(HashSet::new())
                                .insert(this_agent.clone());
                        }
                    }
                }

                match is_loop {
                    true => Some(next_position),
                    false => None,
                }
            })
            .collect::<HashSet<_>>(),
    );

    println!("part_2 => {}", loop_potentials.len() - 1);
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
