use advent::input_store;
use advent_toolbox::spatial::{self, Coordinate, Direction, Space, Traversable};
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

#[derive(Clone, Debug, Default)]
struct Voxel {
    color: Option<String>,
    origin: bool,
    dug: bool,
}

impl Traversable for Voxel {
    fn is_traversable(&self) -> bool {
        self.color.is_none()
    }
}

impl std::fmt::Display for Voxel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.color, self.dug) {
            (None, true) => write!(f, "."),
            (Some(_), true) => {
                let value = if self.origin {
                    "#".to_string().red()
                } else {
                    "#".to_string().white()
                };

                write!(f, "{}", value)
            }
            _ => write!(f, " "),
        }
    }
}

struct Digger {
    position: Coordinate,
}

impl Digger {
    fn apply_instruction(&mut self, inst: &Instruction) -> Vec<(Coordinate, Voxel)> {
        let mut out = Vec::new();

        let direction = match inst.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            _ => inst.direction,
        };

        for _ in 0..inst.distance {
            self.position = self.position + direction;
            out.push((
                self.position,
                Voxel {
                    color: Some(inst.color.clone()),
                    origin: self.position == spatial::ORIGIN,
                    dug: true,
                },
            ));
        }

        out
    }
}

fn dig_lagoon(lagoon: &mut Space<Coordinate, Voxel>, instructions: &Vec<Instruction>) {
    let mut digger = Digger {
        position: Coordinate::new(0, 0),
    };

    for inst in instructions {
        let voxels = digger.apply_instruction(&inst);
        lagoon.extend(voxels);
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

    let mut lagoon: Space<Coordinate, Voxel> = Space::new();
    lagoon.insert(
        Coordinate::new(0, 0),
        Voxel {
            color: Some("origin".to_string()),
            origin: true,
            dug: false,
        },
    );

    dig_lagoon(&mut lagoon, &instructions);

    let point = spatial::ORIGIN + Direction::Up + Direction::Right;
    // let point = spatial::ORIGIN + Direction::Down + Direction::Right;
    let filled = lagoon.flood_fill(&point);
    let part_1 = lagoon.len() + filled.len();
    println!("lagoon:\n{}", lagoon);

    println!("part_1 => {}", part_1);

    let mut lagoon: Space<Coordinate, Voxel> = Space::new();
    let instructions = instructions
        .iter()
        .map(|i| i.true_instruction())
        .collect::<Vec<_>>();

    for inst in &instructions {
        println!("{}", inst);
    }

    dig_lagoon(&mut lagoon, &instructions);
    println!("lagoon is dug, {} edges", lagoon.len());
    let filled = lagoon.flood_fill(&point);
    let part_2 = lagoon.len() + filled.len();

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
