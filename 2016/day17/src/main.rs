use std::{collections::HashSet};

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate, Direction};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

lazy_static::lazy_static! {
    static ref GRID: HashSet<Coordinate> = coordinates_within((0, 0).into(), (3, 3).into()).into_iter().collect();
}

fn digest(input: &str) -> String {
    let d = md5::compute(input);
    format!("{:x}", d)
}

#[derive(Debug, Clone)]
struct Step {
    position: Coordinate,
    password: String,
    path: String,
}

impl Step {
    fn branches(&self) -> Vec<Self> {
        let password = format!("{}{}", self.password, self.path);
        let digest = digest(&password);
        let digest = digest.chars().map(|ch| match ch {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        });

        let out = DIRECTIONS
            .iter()
            .zip(digest)
            .filter(|(dir, open)| GRID.contains(&(self.position + **dir)) && *open)
            .map(|(dir, _)| self.with_direction(*dir))
            .collect();

        out
    }

    fn with_direction(&self, direction: Direction) -> Self {
        let position = self.position + direction;

        let dir_chr = match direction {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::None => panic!("No direction"),
        };

        let path = format!("{}{}", self.path, dir_chr);
        Self {
            position,
            password: self.password.clone(),
            path,
        }
    }

    fn find_path(&self, goal: Coordinate) -> String {
        let mut candiates = vec![self.clone()];
        loop {
            let mut next_candidates = vec![];
            for candidate in candiates {
                let branches = candidate.branches();
                next_candidates.extend(branches);
            }

            for nc in next_candidates.iter() {
                if nc.position == goal {
                    return nc.path.clone();
                }
            }

            candiates = next_candidates;
        }
    }

    fn find_longest_path(&self, goal: Coordinate) -> usize {
        let mut candiates = vec![self.clone()];
        let mut longest = 0;

        loop {
            let mut next_candidates = vec![];
            for candidate in candiates {
                let branches = candidate.branches();
                for branch in branches.into_iter() {
                    if branch.position == goal {
                        if branch.path.len() > longest {
                            longest = branch.path.len();
                        }
                    } else {
                        next_candidates.push(branch);
                    }
                }
            }

            if next_candidates.is_empty() {
                break;
            }

            candiates = next_candidates;
        }

        longest
    }
}

fn main() {
    let input = input_store::get_input(2016, 17).trim().to_string();

    // println!("{:?}", GRID.clone());
    // let input: String = "ihgpwlah".to_string();

    let first_step = Step {
        position: (0, 3).into(),
        password: input.clone(),
        path: "".into(),
    };

    // let path: String = first_step.find_path().unwrap();

    // let mut candiates = vec![first_step];
    // 'outer: loop {
    //     let mut next_candidates = vec![];
    //     for candidate in candiates {
    //         let branches = candidate.branches();
    //         next_candidates.extend(branches);
    //     }

    //     for nc in next_candidates.iter() {
    //         if nc.position == (3, 0).into() {
    //             println!("part_1 => {}", nc.path);
    //             break 'outer;
    //         }
    //     }

    //     candiates = next_candidates;
    // }
    let path = first_step.find_path((3, 0).into());
    println!("part_1 => {}", path);

    let longest = first_step.find_longest_path((3, 0).into());
    println!("part_2 => {}", longest);
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
    #[case("ihgpwlah", "DDRRRD")]
    #[case("kglvqrro", "DDUDRLRRUDRD")]
    #[case("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR")]
    fn p1_tests(#[case] given: &str, #[case] expected: &str) {
        let start = Step {
            position: (0, 3).into(),
            password: given.to_string(),
            path: "".into(),
        };

        let path = start.find_path((3, 0).into());
        assert_eq!(path, expected);
    }

    #[rstest]
    #[case("ihgpwlah", 370)]
    #[case("kglvqrro", 492)]
    #[case("ulqzkmiv", 830)]
    fn p2_tests(#[case] given: &str, #[case] expected: usize) {
        let start = Step {
            position: (0, 3).into(),
            password: given.to_string(),
            path: "".into(),
        };

        let longest = start.find_longest_path((3, 0).into());
        assert_eq!(longest, expected);
    }
}
