use std::collections::HashMap;

use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Space};

const YEAR: usize = 2024;
const DAY: usize = 14;

#[derive(Debug, Clone, PartialEq)]
struct Agent {
    pos: Coordinate,
    dir: Coordinate,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Cell(usize);

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, " "),
            _ => write!(f, "#"),
        }
    }
}
impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "p={},{} v={},{}",
            self.pos.x, self.pos.y, self.dir.x, self.dir.y
        )
    }
}

impl From<&str> for Agent {
    fn from(input: &str) -> Self {
        let input = input.replace("p=", "").replace("v=", "");
        let parts = input.split(" ").collect::<Vec<&str>>();
        let position = parts[0]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let velocity = parts[1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        Self {
            pos: Coordinate::new(position[0], position[1]),
            dir: Coordinate::new(velocity[0], velocity[1]),
        }
    }
}

fn positions_at_time<const X: isize, const Y: isize>(
    agents: &[Agent],
    time: usize,
) -> Vec<Coordinate> {
    let grid_size = Coordinate::new(X, Y);
    agents
        .iter()
        .map(|agent| {
            let pos = agent.pos + agent.dir * time as isize;
            pos % grid_size
        })
        .collect()
}

fn quadrants<const X: isize, const Y: isize>(agents: &[Coordinate]) -> HashMap<Coordinate, usize> {
    let grid_size = Coordinate::new(X, Y);
    let mut quadrants: HashMap<Coordinate, usize> = HashMap::new();

    for agent in agents {
        if grid_size.x % 2 == 1 {
            if agent.x == X / 2 || agent.y == Y / 2 {
                continue;
            }
        }
        let x = if agent.x < X / 2 { 0 } else { 1 };
        let y = if agent.y < Y / 2 { 0 } else { 1 };
        let q = Coordinate::new(x, y);
        *quadrants.entry(q).or_insert(0) += 1;
    }

    quadrants
}

fn g_quadrants<const X: isize, const Y: isize>(
    agents: &Vec<Coordinate>,
    x: usize,
    y: usize,
) -> HashMap<Coordinate, usize> {
    let mut quadrants: HashMap<Coordinate, usize> = HashMap::new();
    for x in 0..x {
        for y in 0..y {
            let q = Coordinate::new(x as isize, y as isize);
            quadrants.entry(q).or_insert(0);
        }
    }

    for agent in agents {
        let q = Coordinate::new(agent.x / x as isize, agent.y / y as isize);
        quadrants.entry(q).and_modify(|e| *e += 1).or_insert(0);
    }
    quadrants
}

fn do_part_1<const X: isize, const Y: isize>(agents: &[Agent], time: usize) -> usize {
    let agents = positions_at_time::<X, Y>(agents, time);
    let quadrants = quadrants::<X, Y>(&agents.as_slice());
    quadrants.iter().map(|(_, v)| v).product::<usize>()
}

fn do_part_2<const X: isize, const Y: isize>(agents: &[Agent], sleep_duration: u64) -> usize {
    let mut time = 7700;

    loop {
        time += 1;
        let agents = positions_at_time::<X, Y>(agents, time);

        let longest_line_x = {
            let mut longest_line = 0;

            for y in 0..Y {
                let mut active = false;
                let mut active_for = 0;
                for x in 0..X {
                    let agent = Coordinate::new(x, y);
                    if agents.contains(&agent) {
                        active = true;
                        active_for += 1;
                    } else {
                        if active {
                            if active_for > longest_line {
                                longest_line = active_for;
                            }
                        }
                        active = false;
                        active_for = 0;
                    }
                }
            }

            longest_line
        };

        // let quadrants = g_quadrants::<X, Y>(&agents, (X / mask.x) as usize, (Y / mask.y) as usize);

        // let full_quadrants = quadrants
        //     .iter()
        //     .filter(|(_, v)| {
        //         let result = **v as f64 / (mask.x * mask.y) as f64 > 0.9;
        //         // println!("{} / ({} * {}) > 0.9 => {}", v, mask.x, mask.y, result);
        //         result
        //     })
        //     .count();

        // if full_quadrants == 0 {
        //     continue;
        // }

        if longest_line_x < 10 {
            continue;
        }

        let mut room: Space<Coordinate, Cell> = Space::new();
        for agent in agents {
            room.entry(agent)
                .and_modify(|e| *e = Cell(e.0 + 1))
                .or_insert(Cell(1));
        }
        // print!("\x1B[2J\x1B[1;1H");
        // let deltas = times_of_high_entropy.windows(2).map(|w| w[1] - w[0]);
        // println!("deltas: {:?}", deltas.collect::<Vec<usize>>());
        // println!("avg_x: {:?}\navg_y: {:?}", avg_x, avg_y);
        // println!("avg_x_delta: {}\navg_y_delta: {}", avg_x_delta, avg_y_delta);
        // println!(
        //     "actual_avg_x: {:?}\nactual_avg_y: {:?}",
        //     actual_avg_x, actual_avg_y
        // );
        // println!("\n\n\n\ntime: {}\n{}", time, room);
        return time;
        // std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3
    // "#;

    let agents = input.lines().map(Agent::from).collect::<Vec<Agent>>();

    // let part_1 = do_part_1::<11, 7>(&agents, 100);
    let part_1 = do_part_1::<101, 103>(&agents, 100);

    println!("part_1 => {}", part_1);

    let part_2 = do_part_2::<101, 103>(&agents, 100);
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
