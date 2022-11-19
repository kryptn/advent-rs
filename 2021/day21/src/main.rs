use std::{collections::HashMap, ops::Add};

use advent::input_store;
use itertools::Itertools;

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

fn part_1(a: usize, b: usize, to: usize) -> usize {
    let mut player_1 = Player { pos: a, score: 0 };
    let mut player_2 = Player { pos: b, score: 0 };

    // let mut player_1 = Player {pos: 4, score: 0};
    // let mut player_2 = Player {pos: 8, score: 0};

    let mut players = [&mut player_1, &mut player_2];

    let mut counter = 0..;
    let mut last_roll = 0;

    while players[1].score < to {
        let a = counter.next().expect("infinite") + 1;
        let b = counter.next().expect("infinite") + 1;
        let c = counter.next().expect("infinite") + 1;
        last_roll = c;

        let roll = (a + b + c) % 100;
        players[0].pos = (players[0].pos + roll) % 10;
        if players[0].pos == 0 {
            players[0].pos = 10;
        }
        players[0].score += players[0].pos;

        players.rotate_left(1);
        //dbg!(&players);
    }

    last_roll * players[0].score
}

#[derive(Debug, Default, Clone)]
struct QuantumPlayer {
    score: usize,
    players: usize,
}

impl QuantumPlayer {
    fn with_points(&self, points: usize) -> Self {
        Self {
            score: self.score + points,
            players: self.players,
        }
    }

    fn add(&mut self, other: &Self) {
        self.score = if other.score > self.score {
            other.score
        } else {
            self.score
        };
        self.players += other.players;
    }
}

#[derive(Debug, Clone)]
struct DiracPlayer {
    position_score: HashMap<usize, QuantumPlayer>,
}

impl DiracPlayer {
    fn new() -> Self {
        Self {
            position_score: HashMap::new(),
        }
    }

    fn score(&self) -> usize {
        self.position_score
            .values()
            .map(|qp| qp.score)
            .max()
            .unwrap()
    }
}

impl From<usize> for DiracPlayer {
    fn from(position: usize) -> Self {
        let player = QuantumPlayer {
            score: 0,
            players: 1,
        };

        let mut position_score = HashMap::new();
        position_score.insert(position, player);

        Self { position_score }
    }
}

fn part_2(a: usize, b: usize, to: usize) -> usize {
    let mut player_1 = DiracPlayer::from(a);
    let mut player_2 = DiracPlayer::from(b);

    let mut players = [&mut player_1, &mut player_2];

    while players[1].score() < to {
        let mut new_dirac = players[0].clone();

        for (pos, player) in &players[0].position_score {
            for c in [1, 2, 3].iter().cloned().combinations_with_replacement(3) {
                let points = {
                    let mut v = pos + c.iter().sum::<usize>();
                    if v > 10 {
                        v = v % 10;
                    }
                    v
                };

                let idx = points % 10;
                let v = new_dirac.position_score.entry(idx).or_default();
                let p_with_points = player.with_points(points);
                //println!("setting idx {}, {:?}, {:?}", idx, &v, &p_with_points);
                v.add(&p_with_points);
                //println!("        now {:?}", &v);
            }
        }

        *players[0] = new_dirac;

        players.rotate_left(1);
        //dbg!(&players);
    }

    //dbg!(&players);

    players[1]
        .position_score
        .values()
        .filter(|qp| qp.score >= to)
        .map(|qp| qp.players)
        .next()
        .unwrap()
}

fn main() {
    let input = input_store::get_input(2021, 21);

    let part_1_score = part_1(10, 2, 1000);
    println!("part_1 => {}", part_1_score);

    let part_2_score = part_2(4, 8, 21);
    println!("part_2 => {}", part_2_score);
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
