use advent::input_store;
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Rank {
    Zero = 0,
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl From<String> for Rank {
    fn from(hand: String) -> Self {
        let counts = hand.chars().counts();
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

fn rank_from_hand_joker(hand: &str) -> Rank {
    let mut max_rank = Rank::Zero;
    for alt in "23456789TQKA".chars() {
        let cards = hand.replace("J", &alt.to_string());
        let rank = Rank::from(cards);
        if rank > max_rank {
            max_rank = rank;
        }
    }
    max_rank
}

fn parse_hand(line: String) -> (String, usize) {
    let mut parts = line.trim().split(" ");
    let hand = parts.next().unwrap().to_string();
    let bet = parts.next().unwrap().to_string();
    (hand, bet.parse::<usize>().unwrap())
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Sortable(Vec<usize>);

#[derive(Debug, Clone)]
struct Part1Sort(String);

impl From<Part1Sort> for Sortable {
    fn from(input: Part1Sort) -> Self {
        let mut out = vec![];
        for c in input.0.chars() {
            match c {
                'A' => out.push(14),
                'K' => out.push(13),
                'Q' => out.push(12),
                'J' => out.push(11),
                'T' => out.push(10),
                _ => out.push(c.to_string().parse::<usize>().unwrap()),
            }
        }
        Self(out)
    }
}

#[derive(Debug, Clone)]
struct Part2Sort(String);

impl From<Part2Sort> for Sortable {
    fn from(input: Part2Sort) -> Self {
        let mut out = vec![];
        for c in input.0.chars() {
            match c {
                'J' => out.push(1),
                'A' => out.push(14),
                'K' => out.push(13),
                'Q' => out.push(12),
                'T' => out.push(10),
                _ => out.push(c.to_string().parse::<usize>().unwrap()),
            }
        }
        Self(out)
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483"#;

    let hands = input
        .trim()
        .lines()
        .map(|line| parse_hand(line.to_string()))
        .collect::<Vec<_>>();

    let part_1 = hands
        .iter()
        .map(|(hand, bet)| {
            (
                Rank::from(hand.clone()),
                Sortable::from(Part1Sort(hand.clone())),
                *bet,
            )
        })
        .sorted()
        .enumerate()
        .map(|(i, item)| (i + 1) * item.2)
        .sum::<usize>();

    println!("part_1 => {}", part_1);

    let part_2 = hands
        .iter()
        .map(|(hand, bet)| {
            (
                rank_from_hand_joker(hand),
                Sortable::from(Part2Sort(hand.clone())),
                *bet,
            )
        })
        .sorted()
        .enumerate()
        .map(|(i, item)| (i + 1) * item.2)
        .sum::<usize>();
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
