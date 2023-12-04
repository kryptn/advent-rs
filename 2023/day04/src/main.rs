use std::collections::{HashMap, HashSet};

use advent::{input_store, parsers::ws};
use nom::{bytes::complete::tag, character::complete::digit1, multi::many1, IResult};

const YEAR: usize = 2023;
const DAY: usize = 4;

#[derive(Default, Clone, Debug)]
struct Card {
    winning: HashSet<u32>,
    selected: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let winning_numbers = &self.selected.intersection(&self.winning).count();

        if winning_numbers == &0 {
            return 0;
        } else {
            2u32.pow(self.winning.intersection(&self.selected).count() as u32 - 1)
        }
    }

    fn matches(&self) -> u32 {
        self.winning.intersection(&self.selected).count() as u32
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = many1(ws(digit1))(input)?;
    Ok((
        input,
        numbers.iter().map(|s| s.parse::<u32>().unwrap()).collect(),
    ))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = ws(digit1)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, winning) = parse_numbers(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, selected) = parse_numbers(input)?;

    Ok((
        input,
        Card {
            winning: winning.into_iter().collect(),
            selected: selected.into_iter().collect(),
        },
    ))
}

fn part_2(cards: &Vec<Card>) -> u32 {
    let mut collection: HashMap<usize, u32> = HashMap::new();
    for x in 0..cards.len() {
        collection.insert(x, 1);
    }

    for (num, card) in cards.iter().enumerate() {
        let start = num + 1;
        let count = collection.get(&num).unwrap().clone();
        for idx in start..start + card.matches() as usize {
            *collection.entry(idx).or_insert(1) += 1 * count;
        }
    }

    collection.values().sum()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let cards: Vec<_> = input
        .lines()
        .map(|line| parse_card(line.trim()).unwrap().1)
        .collect();
    let winnings = cards.iter().map(|card| card.score()).sum::<u32>();

    println!("part_1 => {}", winnings);
    println!("part_2 => {}", part_2(&cards));
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
