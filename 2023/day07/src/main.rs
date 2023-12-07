use std::{cmp::Ordering, collections::HashSet, ffi::c_short};

use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 7;

#[derive(Eq, PartialEq, Hash, Debug, Clone, PartialOrd, Ord)]
struct Card(u8);

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.0 {
            1 => "J".to_string(),
            2..=9 => format!("{}", self.0),
            10 => "T".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => panic!("Invalid card: {}", self.0),
        };
        write!(f, "{}", value)
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Card(1),
            '2' => Card(2),
            '3' => Card(3),
            '4' => Card(4),
            '5' => Card(5),
            '6' => Card(6),
            '7' => Card(7),
            '8' => Card(8),
            '9' => Card(9),
            'T' => Card(10),
            // 'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    // Straight,
    // Flush,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
    // StraightFlush,
    // RoyalFlush,
}

impl From<Hand> for Rank {
    fn from(value: Hand) -> Self {
        let counts = value.cards.iter().counts();
        // println!("value: \"{}\", set: {:?}", value, counts);
        match counts.len() {
            5 => Rank::HighCard,
            4 => Rank::OnePair,
            3 => {
                if counts.iter().any(|c| *c.1 == 3) {
                    Rank::ThreeOfAKind
                } else {
                    Rank::TwoPair
                }
            }
            2 => {
                if counts.iter().any(|c| *c.1 == 4) {
                    Rank::FourOfAKind
                } else {
                    Rank::FullHouse
                }
            }
            1 => Rank::FiveOfAKind,
            _ => panic!("Invalid hand: {:?}", counts),
        }
    }
}

#[derive(Eq, Hash, Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    sorted_cards: Vec<Card>,
    bet: usize,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join("");

        let sorted_cards = self
            .sorted_cards
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join("");
        write!(
            f,
            "{} {} {}   \t {}",
            cards,
            sorted_cards,
            self.bet,
            self.rank()
        )
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let mut parts = value.trim().split(" ");
        let cards: Vec<Card> = parts.next().unwrap().chars().map(|c| c.into()).collect();
        let sorted_cards = cards.iter().cloned().sorted().rev().collect();
        let bet = parts.next().unwrap().parse::<usize>().unwrap();
        Self {
            cards,
            sorted_cards,
            bet,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_cards == other.sorted_cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}
impl Hand {
    fn rank(&self) -> usize {
        let r = Rank::from(self.clone());
        match r {
            Rank::HighCard => 1,
            Rank::OnePair => 2,
            Rank::TwoPair => 3,
            Rank::ThreeOfAKind => 4,
            Rank::FullHouse => 5,
            Rank::FourOfAKind => 6,
            Rank::FiveOfAKind => 7,
        }
    }

    fn level_up(&self) -> usize {
        let hand = self.cards.iter().join("");
        let mut max_rank = 0;
        for alt in "23456789TQKA".chars() {
            let cards = hand.replace("J", &alt.to_string());
            let hand = Hand::from(format!("{} {}", cards, self.bet));
            if hand.rank() > max_rank {
                max_rank = hand.rank();
            }
        };
        max_rank
    }
}

fn print_hands(hands: &Vec<Hand>) {
    for hand in hands {
        println!("{} {:?}", hand, Rank::from(hand.clone()));
    }
    println!("");
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483"#;

    let hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| line.trim().to_string().into())
        .collect::<Vec<_>>();
    print_hands(&hands);

    // let new_hands = hands.iter().cloned().map(|h| (h.rank(), h.sorted_cards, h.bet)).sorted().collect::<Vec<_>>();

    let new_hands = hands.iter().cloned().sorted().collect::<Vec<_>>();
    // dbg!(&new_hands);
    print_hands(&new_hands);

    let scores = new_hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bet * (i + 1))
        .sum::<usize>();

    println!("part_1 => {}", scores);

    let joker_hands = hands
        .iter()
        .cloned()
        .map(|h| (h.level_up(), h.cards, h.bet))
        .sorted()
        .collect::<Vec<_>>();

    let scores = joker_hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.2 * (i + 1))
        .sum::<usize>();

    println!("part_2 => {}", scores);
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
