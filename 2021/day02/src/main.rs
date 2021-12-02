use advent::input_store;

fn main() {
    let input = input_store::get_input(2021, 02);

    let mut x = 0;
    let mut y = 0;

    for line in input.trim().lines() {
        let mut split_line = line.trim().split(" ");
        let cmd = split_line.next().unwrap();
        let value: i32 = split_line.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => unreachable!(),
        }
    }

    println!("part 1 => {}", x * y);

    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    for line in input.trim().lines() {
        let mut split_line = line.trim().split(" ");
        let cmd = split_line.next().unwrap();
        let value: i32 = split_line.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => {
                x += value;
                y += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }

    println!("part 2 => {}", x * y);
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
