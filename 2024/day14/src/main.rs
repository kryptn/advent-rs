use std::collections::HashMap;

use advent::input_store;
use advent_toolbox::{
    spatial::{Coordinate, Space},
};

const YEAR: usize = 2024;
const DAY: usize = 14;

struct Agent {
    pos: Coordinate,
    dir: Coordinate,
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

fn do_part_1<const X: isize, const Y: isize>(agents: &[Agent], time: usize) -> usize {
    let agents = positions_at_time::<X, Y>(agents, time);
    let quadrants = quadrants::<X, Y>(&agents.as_slice());
    quadrants.iter().map(|(_, v)| v).product::<usize>()
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
