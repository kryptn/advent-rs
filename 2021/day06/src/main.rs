use std::collections::{HashMap, VecDeque};

use advent::input_store;

#[derive(Debug)]
struct School {
    fish: [u64; 7],
    staged: [u64; 7],
}

impl From<String> for School {
    fn from(input: String) -> Self {
        let mut fish = [0, 0, 0, 0, 0, 0, 0];
        let staged = [0, 0, 0, 0, 0, 0, 0];

        let timers: Vec<usize> = input
            .trim()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        for timer in timers {
            fish[timer] += 1;
        }

        Self { fish, staged }
    }
}

impl School {
    fn step(&mut self) {
        self.fish.rotate_left(1);
        self.staged.rotate_left(1);

        self.staged[1] = self.fish[0];
        self.fish[0] += self.staged[6];
    }

    fn total(&self) -> u64 {
        let mut total = 0;

        total += self.fish.iter().sum::<u64>();
        total += self.staged[0];

        total
    }
}

fn main() {
    let input = input_store::get_input(2021, 06);
    //let input = "3,4,3,1,2".to_string();

    let mut school: School = input.clone().into();

    for _ in 0..80 {
        school.step();
    }

    println!("part_1 => {}", school.total());

    for _ in 80..256 {
        school.step();
    }

    println!("part_2 => {}", school.total());
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
