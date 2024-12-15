use std::collections::VecDeque;

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Direction, Space};
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 15;

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    #[default]
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '@' => Self::Robot,
            '#' => Self::Wall,
            'O' => Self::Box,
            '.' => Self::Empty,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => panic!("invalid cell"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Robot => write!(f, "@"),
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
            Self::Empty => write!(f, "."),
            Self::BoxLeft => write!(f, "["),
            Self::BoxRight => write!(f, "]"),
        }
    }
}

fn connected_cells(
    warehouse: &Space<Coordinate, Cell>,
    pos: Coordinate,
    direction: Direction,
) -> Option<Vec<Coordinate>> {
    let mut cells = vec![];
    let mut queue: VecDeque<Coordinate> = vec![pos].into();
    while let Some(cell) = queue.pop_front() {
        match warehouse.get(&cell) {
            Some(Cell::Box) => {
                cells.push(cell);
                queue.push_back(cell + direction);
            }
            Some(Cell::BoxLeft) | Some(Cell::BoxRight) => {
                cells.push(cell);
                if direction == Direction::Up || direction == Direction::Down {
                    let other = match warehouse.get(&cell) {
                        Some(Cell::BoxLeft) => cell + Direction::Right,
                        Some(Cell::BoxRight) => cell + Direction::Left,
                        _ => panic!("this shouldn't happen"),
                    };
                    cells.push(other);
                    queue.push_back(other + direction);
                }
                queue.push_back(cell + direction);
            }
            Some(Cell::Empty) => {}
            Some(Cell::Wall) => return None,
            Some(Cell::Robot) | None => panic!("this shouldn't happen"),
        }
    }

    Some(cells.into_iter().unique().collect())
}

fn robot_step(
    warehouse: &mut Space<Coordinate, Cell>,
    robot: Coordinate,
    direction: Direction,
) -> Option<Coordinate> {
    let next = robot + direction;
    match warehouse.get(&next) {
        Some(Cell::Empty) => Some(next),
        Some(Cell::Wall) => None,

        Some(Cell::Box) | Some(Cell::BoxLeft) | Some(Cell::BoxRight) => {
            let connected = connected_cells(warehouse, next, direction);
            if let Some(connected) = connected {
                // extract all connected cells, we're pushing those
                let next_cells: Vec<_> = connected
                    .iter()
                    .map(|c| (*c, warehouse.insert(*c, Cell::Empty).unwrap()))
                    .collect();

                // move them
                for (pos, cell) in next_cells {
                    warehouse.insert(pos + direction, cell);
                }

                // move the robot
                Some(next)
            } else {
                None
            }
        }
        Some(Cell::Robot) | None => {
            panic!("don't think this should happen")
        }
    }
}

fn score(warehouse: &Space<Coordinate, Cell>) -> isize {
    let mut score = 0;
    for (pos, _) in warehouse.iter().filter(|(_, c)| match c {
        Cell::Box | Cell::BoxLeft => true,
        _ => false,
    }) {
        score += pos.y * 100 + pos.x;
    }
    score
}

fn handle(warehouse: &mut Space<Coordinate, Cell>, directions: &Vec<Direction>) -> isize {
    let mut robot = warehouse
        .iter()
        .find(|(_, c)| **c == Cell::Robot)
        .unwrap()
        .0
        .clone();
    warehouse.insert(robot, Cell::Empty);

    for direction in directions {
        let next = robot_step(warehouse, robot, *direction);
        if let Some(next) = next {
            robot = next;
        }

        // let direction_char = match direction {
        //     Direction::Down => '^',
        //     Direction::Up => 'v',
        //     Direction::Right => '>',
        //     Direction::Left => '<',
        //     _ => panic!("invalid direction"),
        // };

        // warehouse.insert(robot, Cell::Robot);
        // println!("Move: {direction_char}\n{warehouse}");
        // std::thread::sleep(std::time::Duration::from_millis(250));
        // warehouse.insert(robot, Cell::Empty);
    }
    score(warehouse)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut warehouse: Space<Coordinate, Cell> = input[0].into();

    let directions: Vec<Direction> = input[1]
        .chars()
        .filter_map(|c| match Direction::from(c) {
            Direction::None => None,
            Direction::Down => Some(Direction::Up),
            Direction::Up => Some(Direction::Down),
            d => Some(d),
        })
        .collect();

    let part_1 = handle(&mut warehouse, &directions);
    println!("part_1 => {}", part_1);

    let mut warehouse_p2 = input[0]
        .trim()
        .chars()
        .map(|c| match c {
            '@' => "@.",
            '#' => "##",
            '.' => "..",
            'O' => "[]",
            '\n' => "\n",
            _ => "",
        })
        .collect::<Vec<&str>>()
        .join("")
        .into();

    let part_2 = handle(&mut warehouse_p2, &directions);
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
