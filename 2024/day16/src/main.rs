use std::{cmp, convert::TryInto};

use advent::input_store;
use advent_toolbox::{
    algo::{dijkstra_all, dijkstra_all_paths},
    spatial::{Agent, Coordinate, Direction, Space, DOWN, LEFT, RIGHT, UP},
};
use itertools::Itertools;
use petgraph::{
    algo::{all_simple_paths, dijkstra},
    prelude::DiGraphMap,
};

const YEAR: usize = 2024;
const DAY: usize = 16;

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    #[default]
    Wall,

    Empty,
    Visited(Direction),
    Start,
    End,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("invalid cell"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
            Self::Visited(Direction::Up) => write!(f, "v"),
            Self::Visited(Direction::Down) => write!(f, "^"),
            Self::Visited(Direction::Left) => write!(f, "<"),
            Self::Visited(Direction::Right) => write!(f, ">"),
            Self::Visited(Direction::None) => write!(f, "."),
        }
    }
}

fn edges_for(agent: &Agent, maze: &Space<Coordinate, Cell>) -> Vec<(Agent, usize)> {
    let mut out = vec![];

    let ahead = agent.position + agent.direction;
    let to_left = agent.strafe_left().position;
    let to_right = agent.strafe_right().position;

    let ahead_cell = maze.get(&ahead).unwrap();
    let left_cell = maze.get(&to_left).unwrap();
    let right_cell = maze.get(&to_right).unwrap();

    match (left_cell, ahead_cell, right_cell) {
        (Cell::Wall, Cell::Wall, Cell::Wall) => return out,
        (Cell::Wall, Cell::Wall, Cell::Empty | Cell::End) => {
            out.push((agent.turn_right(), 1000));
        }
        (Cell::Empty | Cell::End, Cell::Wall, Cell::Wall) => {
            out.push((agent.turn_left(), 1000));
        }
        (Cell::Empty | Cell::End, Cell::Wall, Cell::Empty | Cell::End) => {
            out.push((agent.turn_left(), 1000));
            out.push((agent.turn_right(), 1000));
        }
        (l, Cell::Empty | Cell::End, r) => {
            out.push((agent.forward(), 1));
            if matches!(l, Cell::Empty | Cell::End) {
                out.push((agent.turn_left(), 1000));
            }
            if matches!(r, Cell::Empty | Cell::End) {
                out.push((agent.turn_right(), 1000));
            }
        }
        _ => {}
    }

    // if ahead_cell == &Cell::End || ahead_cell == &Cell::Empty {
    //     out.push((
    //         Agent {
    //             position: ahead,
    //             direction: agent.direction,
    //         },
    //         1,
    //     ));
    //     return out;
    // }

    // match maze.get(&ahead) {
    //     Some(Cell::Wall) => match (maze.get(&to_left), maze.get(&to_right)) {
    //         (Some(Cell::Wall), Some(Cell::Wall)) => return out,
    //         (Some(Cell::Wall), _) => {
    //             out.push((agent.turn_right(), 1000));
    //             return out;
    //         }
    //         _ => {}
    //     },
    //     Some(Cell::Empty) | Some(Cell::End) => return out,
    //     _ => {}
    // }

    // if maze.get(&move_forward) == Some(&Cell::Empty) || maze.get(&move_forward) == Some(&Cell::End)
    // {
    //     out.push((
    //         Agent {
    //             position: move_forward,
    //             direction: agent.direction,
    //         },
    //         1,
    //     ));
    //     let left = agent.strafe_left().position;
    //     let right = agent.strafe_right().position;
    //     if maze.get(&left) == Some(&Cell::Wall) && maze.get(&right) == Some(&Cell::Wall) {
    //         return out;
    //     }
    // }
    // out.push((
    //     Agent {
    //         position: agent.position,
    //         direction: agent.direction.turn_left(),
    //     },
    //     1000,
    // ));
    // out.push((
    //     Agent {
    //         position: agent.position,
    //         direction: agent.direction.turn_right(),
    //     },
    //     1000,
    // ));

    out
}

fn make_graph(maze: &Space<Coordinate, Cell>) -> DiGraphMap<Agent, usize> {
    let mut g: DiGraphMap<Agent, usize> = DiGraphMap::new();

    for cell in maze.iter().filter(|(k, v)| **v != Cell::Wall) {
        for direction in [UP, DOWN, LEFT, RIGHT] {
            let this = Agent {
                position: *cell.0,
                direction,
            };
            for (next, cost) in edges_for(&this, maze) {
                g.add_edge(this, next, cost);
            }
        }
    }

    g
}

fn dfs(
    graph: &DiGraphMap<Agent, usize>,
    start: Agent,
    is_goal: impl Fn(&Agent) -> bool,
    heuristic: impl Fn(&DiGraphMap<Agent, usize>, &Agent) -> usize,
    show_path: impl Fn(&Vec<Agent>),
    given_min_cost: Option<usize>,
) -> Vec<(Vec<Agent>, usize)> {
    let mut stack = vec![(vec![start], 0)];
    let mut paths = vec![];

    let mut min_cost = given_min_cost.unwrap_or(std::usize::MAX);
    let mut i = 0;

    while let Some((path, cost)) = stack.pop() {
        // show_path(&path);
        // println!("stack: {:?}", stack);
        // std::thread::sleep(std::time::Duration::from_millis(100));

        i += 1;

        if i % 100000 == 0 {
            println!("loop {i}");
            println!("{} solutions at cost {min_cost}", paths.len());
            println!("{} paths in stack", stack.len());
        }

        if cost > min_cost {
            continue;
        }
        let current = path.last().unwrap();

        if is_goal(current) {
            if cost < min_cost {
                min_cost = cost;
                paths.clear();
            }
            paths.push((path, cost));
        } else {
            if cost + heuristic(graph, current) > min_cost {
                continue;
            }
            // let neighbors = graph
            //     .neighbors(*current)
            //     .sorted_by_key(|n| std::cmp::Reverse(heuristic(graph, current, n)));

            // for neighbor in neighbors {
            //     if path.contains(&neighbor) {
            //         continue;
            //     }
            //     let new_cost = cost + graph.edge_weight(*current, neighbor).unwrap();
            //     // let new_cost = cost + 1;
            //     let mut new_path = path.clone();
            //     new_path.push(neighbor);
            //     stack.push((new_path, new_cost));
            // }

            let new_paths = graph
                .neighbors(*current)
                .filter(|n| {
                    !path[0..path.len() - 1]
                        .iter()
                        .any(|p| p.position == n.position)
                })
                // .filter(|n| !path.iter().any(|p| p.position == n.position))
                .map(|n| {
                    let new_cost = cost + graph.edge_weight(*current, n).unwrap();
                    let mut new_path = path.clone();
                    new_path.push(n);
                    (new_path, new_cost)
                })
                // .sorted_by_key(|(_, c)| std::cmp::Reverse(*c))
                .collect::<Vec<_>>();

            // dbg!(&new_paths);

            stack.extend(new_paths);
        }
    }

    paths
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let input = r#"###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    "#;

    let maze: Space<Coordinate, Cell> = input.into();

    let start = maze.find(|v| *v == Cell::Start).unwrap();
    let end = maze.find(|v| *v == Cell::End).unwrap();

    let g = make_graph(&maze);

    let edges = |n: &(Agent, usize)| -> Vec<((Agent, usize), Option<usize>)> {
        edges_for(&n.0, &maze)
            .into_iter()
            .map(|(a, b)| ((a, n.1 + b), Some(b)))
            .collect()
    };
    let is_goal = |n: &(Agent, usize)| n.0.position == end;
    // let cost_fn = |c: &(Agent, Action)| match c.1 {
    //     Action::Step => Some(1),
    //     Action::TurnLeft => Some(1000),
    //     Action::TurnRight => Some(1000),
    //     Action::Start => Some(0),
    // };

    // let initial = vec![(
    //     Agent {
    //         position: start,
    //         direction: RIGHT,
    //     },
    //     Action::Start,
    // )];

    let initial = Agent {
        position: start,
        direction: RIGHT,
    };

    // g.neighbors(initial).for_each(|n| {
    //     println!("{:?}", n);
    // });

    // dbg!(&g.edges(initial));
    let goals = [DOWN, RIGHT]
        .iter()
        .map(|d| Agent {
            position: end,
            direction: *d,
        })
        .collect::<Vec<_>>();

    let result = dijkstra(&g, initial, None, |e| *e.2);

    // dbg!(&result);

    let part_1 = result
        .iter()
        .filter(|(k, _)| k.position == end)
        .min_by_key(|v| v.1)
        .unwrap()
        .1;
    println!("part_1 => {}", part_1);

    let heuristic = |g: &DiGraphMap<Agent, usize>, next_node: &Agent| -> usize {
        let dist = next_node.position.distance(&end);

        if next_node.position.x == end.x && next_node.direction == DOWN
            || next_node.position.y == end.y && next_node.direction == RIGHT
        {
            dist
        } else if next_node.position.x != end.x && next_node.position.y != end.y {
            if next_node.direction == DOWN || next_node.direction == RIGHT {
                dist + 1000
            } else {
                dist + 2000
            }
        } else {
            dist
        }
    };

    let show_path = |path: &Vec<Agent>| {
        let mut maze = maze.clone();
        for a in path {
            maze.insert(a.position, Cell::Visited(a.direction.try_into().unwrap()));
        }
        println!("{}\n\n", maze);
    };

    let paths = dijkstra_all_paths(&[(initial, 0)], edges, is_goal);

    let all_cells = paths
        .iter()
        .flatten()
        .map(|a| a.0.position)
        .unique()
        .collect::<Vec<_>>();

    // let all_paths = dfs(
    //     &g,
    //     initial,
    //     |a| a.position == end,
    //     heuristic,
    //     show_path,
    //     Some(*part_1),
    // );

    // let all_cells = all_paths
    //     .iter()
    //     .map(|(p, _)| p)
    //     .flatten()
    //     .map(|a| a.position)
    //     .unique()
    //     .collect::<Vec<_>>();

    // dbg!(&all_paths);
    // dbg!(&all_paths.len());

    println!("part_2 => {}", all_cells.len());
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
