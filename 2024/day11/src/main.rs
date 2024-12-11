use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 11;

#[derive(Debug, PartialEq)]
struct Stone(u64);

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Stone {
    fn next(&self) -> Vec<Stone> {
        let digits = (self.0 as f64).log10().floor() as u64 + 1;
        // dbg!(digits);
        if self.0 == 0 {
            return vec![Stone(1)];
        } else if self.0 >= 10 && digits % 2 == 0 {
            // lol 1 would give me 0 digits
            let middle = 10u64.pow((digits as u32) / 2);
            let left = self.0 / middle;
            let right = self.0 % middle;
            return vec![Stone(left), Stone(right)];
        } else {
            return vec![Stone(self.0 * 2024)];
        }
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let mut rocks = input.trim().split(" ").map(|x| Stone(x.parse().unwrap())).collect::<Vec<Stone>>();


    const FIRST: i32 = 25;
    for n in 1..=FIRST {

        rocks = rocks.iter().flat_map(|x| x.next()).collect();
        println!("blinks: {n}, rocks.len: {}", rocks.len());
        println!("rocks: {}\n", rocks.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
    println!("part_1 => {}", rocks.len());

    // lol this will not work
    for n in 1..=50 {
        rocks = rocks.iter().flat_map(|x| x.next()).collect();
        println!("blinks: {}, rocks: {}", n + FIRST, rocks.len());
    }


    println!("part_2 => {}", rocks.len());
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
    #[case(0, vec![Stone(1)])]
    #[case(1, vec![Stone(2024)])]
    #[case(2, vec![Stone(4048)])]
    #[case(10, vec![Stone(1), Stone(0)])]
    #[case(11, vec![Stone(1), Stone(1)])]
    #[case(22, vec![Stone(2), Stone(2)])]
    #[case(111, vec![Stone(111 * 2024)])]
    #[case(1000, vec![Stone(10), Stone(0)])]
    #[case(2222, vec![Stone(22), Stone(22)])]
    #[case(22223, vec![Stone(22223 * 2024)])]
    #[case(222222, vec![Stone(222), Stone(222)])]
    fn p1_tests(#[case] given: u64, #[case] expected: Vec<Stone>) {

        let next_stone = Stone(given).next();
        dbg!(&next_stone);
        assert_eq!(next_stone, expected);

    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
