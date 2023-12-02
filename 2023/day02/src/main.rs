use std::ops::Add;

use advent::{input_store, parsers::ws};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult,
};

const YEAR: usize = 2023;
const DAY: usize = 2;

#[derive(Default, Clone, Debug)]
struct Hand {
    red: usize,
    blue: usize,
    green: usize,
}

impl Add for Hand {
    type Output = Hand;

    fn add(self, other: Hand) -> Hand {
        Hand {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl Hand {
    fn valid(&self, given: &Hand) -> bool {
        self.red <= given.red && self.blue <= given.blue && self.green <= given.green
    }
}

fn parse_color(input: &str) -> IResult<&str, Hand> {
    let (input, count) = digit1(input)?;
    let (input, color) = alt((ws(tag("red")), ws(tag("blue")), ws(tag("green"))))(input)?;

    let mut hand = Hand {
        red: 0,
        blue: 0,
        green: 0,
    };

    match color {
        "red" => hand.red = count.parse::<usize>().unwrap(),
        "blue" => hand.blue = count.parse::<usize>().unwrap(),
        "green" => hand.green = count.parse::<usize>().unwrap(),
        _ => panic!("Unknown color: {}", color),
    }

    Ok((input, hand))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    // 3 blue, 4 red

    let (input, hands) = separated_list1(tag(", "), parse_color)(input)?;
    let hand = hands
        .into_iter()
        .fold(Hand::default(), |acc, hand| acc + hand);
    Ok((input, hand))
}

fn parse_game(input: &str) -> IResult<&str, (usize, Vec<Hand>)> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    let (input, _) = tag("Game ")(input)?;
    let (input, game_number) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, hands) = separated_list1(tag("; "), parse_hand)(input)?;

    Ok((input, (game_number.parse::<usize>().unwrap(), hands)))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    //                      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    //                      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    //                      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    //                      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let items = input
        .trim()
        .lines()
        .map(|line| parse_game(line.trim()).unwrap().1)
        .collect::<Vec<_>>();
    // dbg!(items);

    let given = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let part_1 = items
        .iter()
        .filter(|(_, hands)| hands.iter().all(|h| h.valid(&given)))
        .map(|(id, _)| id)
        .sum::<usize>();

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
