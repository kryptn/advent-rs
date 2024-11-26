use advent::input_store;
use advent_toolbox::spatial::Coordinate3d;
use rgeometry::data::Line;

const YEAR: usize = 2023;
const DAY: usize = 24;

struct Object {
    position: Coordinate3d,
    velocity: Coordinate3d,
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        let value = value.trim().replace("@ ", "").replace(",", "");
        let mut parts = value.split_ascii_whitespace();
        let x = parts.next().unwrap().parse::<isize>().unwrap();
        let y = parts.next().unwrap().parse::<isize>().unwrap();
        let z = parts.next().unwrap().parse::<isize>().unwrap();

        let v_x = parts.next().unwrap().parse::<isize>().unwrap();
        let v_y = parts.next().unwrap().parse::<isize>().unwrap();
        let v_z = parts.next().unwrap().parse::<isize>().unwrap();

        Self {
            position: Coordinate3d { x, y, z },
            velocity: Coordinate3d {
                x: v_x,
                y: v_y,
                z: v_z,
            },
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    println!("part_1 => {}", "not done");
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
