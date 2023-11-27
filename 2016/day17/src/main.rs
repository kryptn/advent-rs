use std::collections::HashSet;

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate, Direction};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

lazy_static::lazy_static! {
    static ref GRID: HashSet<Coordinate> = coordinates_within((0, 0).into(), (4, 4).into()).into_iter().collect();
}

fn digest(input: &str) -> String {
    let d = md5::compute(input);
    format!("{:x}", d)
}

struct Step {
    position: Coordinate,
    password: String,
}

impl Step {
    fn branches(&self) -> Vec<Self> {
        let mut out = vec![];
        let digest = digest(&self.password);
        let digest = digest.chars().map(|ch| match ch {
            'b' | 'c' | 'd' | 'f' => true,
            _ => false,
        });

        let out = DIRECTIONS
            .iter()
            .zip(digest)
            .filter(|(dir, open)| GRID.contains(self.position + dir) && *open)
            .map(|(dir, _)| {
                let ns = self.clone();
                self.position + *dir
            })
            .collect();

        out
    }
}

fn main() {
    let input = input_store::get_input(2016, 17).trim().to_string();

    let first_step = Step {
        position: (0, 3).into(),
        password: input.clone(),
    };

    first_step.branches();
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
