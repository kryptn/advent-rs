use advent::input_store;
use itertools::zip_eq;

#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    fn for_condition(&self, condition: &Condition) -> Self {
        match (self, condition) {
            (Hand::Rock, Condition::Lose) => Self::Scissors,
            (Hand::Rock, Condition::Draw) => Self::Rock,
            (Hand::Rock, Condition::Win) => Self::Paper,
            (Hand::Paper, Condition::Lose) => Self::Rock,
            (Hand::Paper, Condition::Draw) => Self::Paper,
            (Hand::Paper, Condition::Win) => Self::Scissors,
            (Hand::Scissors, Condition::Lose) => Self::Paper,
            (Hand::Scissors, Condition::Draw) => Self::Scissors,
            (Hand::Scissors, Condition::Win) => Self::Rock,
        }
    }
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Condition {
    Lose,
    Draw,
    Win,
}

impl From<&Hand> for Condition {
    fn from(hand: &Hand) -> Self {
        match hand {
            Hand::Rock => Self::Lose,
            Hand::Paper => Self::Draw,
            Hand::Scissors => Self::Win,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Match {
    them: Hand,
    me: Hand,
}

const DRAW: u32 = 3;
const LOSE: u32 = 0;
const WIN: u32 = 6;

impl Match {
    fn resolve(&self) -> u32 {
        let value = match (self.them, self.me) {
            (Hand::Rock, Hand::Rock) => DRAW,
            (Hand::Rock, Hand::Paper) => WIN,
            (Hand::Rock, Hand::Scissors) => LOSE,
            (Hand::Paper, Hand::Rock) => LOSE,
            (Hand::Paper, Hand::Paper) => DRAW,
            (Hand::Paper, Hand::Scissors) => WIN,
            (Hand::Scissors, Hand::Rock) => WIN,
            (Hand::Scissors, Hand::Paper) => LOSE,
            (Hand::Scissors, Hand::Scissors) => DRAW,
        };

        value + self.me.points()
    }
}

impl From<(Hand, Hand)> for Match {
    fn from((them, me): (Hand, Hand)) -> Self {
        Self { them, me }
    }
}

fn main() {
    let input = input_store::get_input(2022, 02);

    let items: Vec<(Hand, Hand)> = input
        .lines()
        .map(|line| {
            let hands: Vec<Hand> = line.trim().split_whitespace().map(|v| v.into()).collect();
            (hands[0], hands[1])
        })
        .collect();

    let (them, me) = {
        let mut them = Vec::new();
        let mut me = Vec::new();
        for (theirs, mine) in items {
            them.push(theirs);
            me.push(mine);
        }
        (them, me)
    };

    let part_1: u32 = zip_eq(&them, &me)
        .map(|(t, m)| Match::from((*t, *m)).resolve())
        .sum();

    println!("part_1 => {}", part_1);

    let conditions: Vec<Condition> = me.iter().map(Condition::from).collect();

    let part_2: u32 = zip_eq(&them, &conditions)
        .map(|(t, c)| {
            let m = t.for_condition(c);
            Match::from((*t, m)).resolve()
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
