use advent::input_store;

const YEAR: usize = 2025;
const DAY: usize = 01;



fn main() {
    let input = input_store::get_input(YEAR, DAY);

//     let input = r#"L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82"#;

    let turns = input.trim().lines().map(|line| {
        let direction = line.chars().next().unwrap();
        let direction = match direction {
            'R' => 1,
            'L' => -1,
            _ => panic!("unknown direction"),
        };
        let distance: isize = line[1..].parse().unwrap();

        (direction, distance)
    });

    let mut pos = 50;

    let mut times_at_zero = 0;
    let mut times_at_zero_at_all = 0;


    // for (turn, distance) in turns {
    //     let pre_mod = distance * turn + pos;
    //     pos = pre_mod % 100;

    //     if pre_mod != pos {
    //         times_at_zero_at_all += 1;
    //     }

    //     if pos == 0 {
    //         times_at_zero += 1;
    //     }

    // }

    for (turn, distance) in turns {
        let mut tmp = pos;
        for _ in 0..distance {
            tmp = (tmp + turn) % 100;

            if tmp == 0 {
                times_at_zero_at_all += 1;
            }
        }
        pos = tmp % 100;
        if pos == 0 {
            times_at_zero += 1;
        }
    }


    println!("part_1 => {}", times_at_zero);
    println!("part_2 => {}", times_at_zero_at_all);
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
