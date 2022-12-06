use advent::input_store;
use advent_toolbox::rps::{Condition, Hand};

fn parse<'a, T, U>(line: &'a str) -> (T, U)
where
    T: From<&'a str>,
    U: From<&'a str>,
{
    let items: Vec<&str> = line.trim().split_whitespace().collect();
    (items[0].into(), items[1].into())
}

fn points_for_hand(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn points_for_condition(condition: &Condition) -> u32 {
    match condition {
        Condition::Lose => 0,
        Condition::Draw => 3,
        Condition::Win => 6,
    }
}

fn main() {
    let input = input_store::get_input(2022, 02);

    let items: Vec<(Hand, Hand)> = input.lines().map(|line| parse(line)).collect();
    let part_1: u32 = items
        .iter()
        .map(|(them, me)| {
            let cond = them.against(me);
            points_for_condition(&cond) + points_for_hand(&me)
        })
        .sum();
    println!("part_1 => {}", part_1);

    let items: Vec<(Hand, Condition)> = input.lines().map(|line| parse(line)).collect();
    let part_2: u32 = items
        .iter()
        .map(|(them, cond)| {
            let me = them.desired_condition(cond);
            points_for_condition(&cond) + points_for_hand(&me)
        })
        .sum();
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
