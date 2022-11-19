
use advent::{input_store, space::{Coordinate2d, Path, Point, Traversal}};

struct Step {
    position: Coordinate2d,
    password: String,
}

impl Traversal<Coordinate2d> for Step {
    fn next_steps(&self, visited: &Path<Coordinate2d>) -> Vec<Coordinate2d> {
        let current  = visited.path.iter().last().unwrap();

        current.cardinals().iter().cloned().filter(|&c| )




        todo!()
    }

    fn at_goal(&self, visited: &Path<Coordinate2d>) -> bool {
        todo!()
    }
}

fn digest(input: &str) -> String {
    let d = md5::compute(input);
    format!("{:x}", d)
}

fn main() {
    let input = input_store::get_input(2016, 17);
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
