use advent::input_store;
use advent_toolbox::spatial::{Coordinate, Direction, Space};

const YEAR: usize = 2025;
const DAY: usize = 07;

#[derive(Clone, Debug, Default)]
enum Platform {
    #[default]
    Empty,
    Splitter,
    Source,
    Beam,
}

impl From<char> for Platform {
    fn from(c: char) -> Self {
        match c {
            '.' => Platform::Empty,
            '^' => Platform::Splitter,
            'S' => Platform::Source,
            '|' => Platform::Beam,
            _ => panic!("unknown platform char {}", c),
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Platform::Empty => '.',
            Platform::Splitter => '^',
            Platform::Source => 'S',
            Platform::Beam => '|',
        };
        write!(f, "{}", c)
    }
}

const TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = TEST_INPUT;

    let space: Space<Coordinate, Platform> = Space::from_lines(&input);

    let part_1 = space
        .iter()
        .filter(|(_, p)| matches!(p, Platform::Splitter))
        .filter(|(c, _)| {
            let mut pos = c.down();
            let mut visited = vec![];
            let mut val = false;
            while space.contains_key(&pos) {
                visited.push(pos);
                let this = space.get(&pos).unwrap();
                match this {
                    Platform::Source => {
                        val = true;
                        break;
                    }
                    Platform::Splitter => break,
                    _ => {}
                }
                let left = space.get(&pos.left()).unwrap();
                let right = space.get(&pos.right()).unwrap();

                match (left, right) {
                    (Platform::Splitter, _) | (_, Platform::Splitter) => {
                        val = true;
                        break;
                    }
                    _ => {}
                }
                pos = pos.down();
            }

            // let mut dbg_space = space.clone();
            // for v in visited {
            //     dbg_space.insert(v, Platform::Beam);
            // }
            // println!("{}\n\n\n", dbg_space);

            val
        })
        .count();

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
