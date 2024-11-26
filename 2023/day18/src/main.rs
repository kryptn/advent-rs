use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Direction, Line, Space, Traversable};
use colored::Colorize;

const YEAR: usize = 2023;
const DAY: usize = 18;

struct Instruction {
    direction: Direction,
    distance: usize,
    color: String,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?} {} {}", self.direction, self.distance, self.color);
        write!(f, "{}", value)
    }
}

impl Instruction {
    fn true_instruction(&self) -> Self {
        let distance = {
            let rd = &self.color[1..6];
            u64::from_str_radix(rd, 16).unwrap() as usize
        };
        let direction = match self.color.chars().last().unwrap() {
            '0' => Direction::Up,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Right,
            _ => panic!("bad direction"),
        };

        Self {
            direction,
            distance,
            color: self.color.clone(),
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let value = value.trim().replace("(", "").replace(")", "");
        let parts = value.split_whitespace().collect::<Vec<&str>>();

        let direction = Direction::from(parts[0].to_string());
        let distance = parts[1].parse::<usize>().unwrap();
        let color = parts[2].to_string();

        Self {
            direction,
            distance,
            color,
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let input = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)"#;

    let instructions: Vec<_> = input
        .trim()
        .lines()
        .map(|l| Instruction::from(l.to_string()))
        .collect();

    let mut pos = Coordinate::new(0, 0);
    let edges: Vec<_> = instructions
        .iter()
        .map(|i| {
            let end = pos + i.direction * i.distance as isize;
            let out = (pos.clone(), end);
            pos = end;
            out
        })
        .collect();

    for (start, end) in &edges {
        println!("{:?} -> {:?}", start, end);
    }

    let line_1 = (Coordinate::new(-5, 5), Coordinate::new(5, 5));
    let line_2 = (Coordinate::new(0, -5), Coordinate::new(0, 5));

    let intersection_point = line_1.intersection(&line_2);

    println!("intersection point: {:?}", intersection_point);
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
