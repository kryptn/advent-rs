use std::{
    iter::Sum,
    ops::Add,
    slice::Iter,
    sync::mpsc::{channel, Receiver},
    thread::spawn,
};

use advent::{fetch, numbers::factors};
use itertools::{Combinations, Itertools, Powerset};

#[derive(Clone, Copy, Debug)]
struct Stats {
    cost: i32,
    attack: i32,
    defense: i32,
}

impl Stats {
    fn damage_from(&self, other: &Stats) -> i32 {
        let d = other.attack - self.defense;
        if d < 1 {
            1
        } else {
            d
        }
    }
}

impl Add<Stats> for Stats {
    type Output = Stats;

    fn add(self, rhs: Stats) -> Self::Output {
        Stats {
            cost: self.cost + rhs.cost,
            attack: self.attack + rhs.attack,
            defense: self.defense + rhs.defense,
        }
    }
}

impl Add<&Stats> for Stats {
    type Output = Stats;

    fn add(self, rhs: &Stats) -> Self::Output {
        Stats {
            cost: self.cost + rhs.cost,
            attack: self.attack + rhs.attack,
            defense: self.defense + rhs.defense,
        }
    }
}

#[derive(Clone, Debug)]
struct Entity {
    health: i32,
    stats: Stats,
    player: bool,
}

impl Entity {
    fn new(health: i32, stats: Stats, player: bool) -> Self {
        Self {
            health,
            stats,
            player,
        }
    }

    fn wins_against(self, other: Self) -> bool {
        let mut players = vec![self, other];

        while players[0].health > 0 {
            let attacker = players[0].clone();
            let defender = players[1].clone();

            let new_defender = Self {
                health: defender.health - defender.stats.damage_from(&attacker.stats),
                stats: players[1].stats,
                player: players[1].player,
            };
            players.pop().unwrap();
            players.push(new_defender);
            players.rotate_left(1);
        }

        !players[0].player
    }
}

fn loadouts() -> Vec<Stats> {
    let mut out = Vec::new();

    for &weapon in WEAPONS {
        for &armor in ARMOR {
            for rings in RINGS.iter().combinations(2) {
                out.push(weapon + armor + rings[0] + rings[1]);
            }
        }
    }

    out
}

fn main() {
    let input = fetch::get_input(2015, 20);

    let mut min_cost = 10000;
    let mut most_lost = 0;

    let enemy: Entity = Entity::new(
        103,
        Stats {
            cost: 0,
            attack: 9,
            defense: 2,
        },
        false,
    );

    for candidate in loadouts() {
        let player = Entity::new(100, candidate.clone(), true);
        if player.wins_against(enemy.clone()) {
            if candidate.cost < min_cost {
                // dbg!(&player);
                // dbg!(&enemy);
                // println!("\n\n\n\n");

                //dbg!(candidate);
                min_cost = candidate.cost;
            }
        } else {
            if candidate.cost > most_lost {
                most_lost = candidate.cost;
            }
        }
    }

    println!("part 1 => {}", min_cost);
    println!("part 2 => {}", most_lost);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {
        let player = Entity::new(
            8,
            Stats {
                cost: 0,
                attack: 5,
                defense: 5,
            },
            true,
        );
        let enemy = Entity::new(
            12,
            Stats {
                cost: 0,
                attack: 7,
                defense: 2,
            },
            false,
        );

        assert_eq!(player.wins_against(enemy), true);
    }

    #[test]
    fn p2_tests() {}
}

// Weapons:    Cost  Damage  Armor
// Dagger        8     4       0
// Shortsword   10     5       0
// Warhammer    25     6       0
// Longsword    40     7       0
// Greataxe     74     8       0

// Armor:      Cost  Damage  Armor
// Leather      13     0       1
// Chainmail    31     0       2
// Splintmail   53     0       3
// Bandedmail   75     0       4
// Platemail   102     0       5

// Rings:      Cost  Damage  Armor
// Damage +1    25     1       0
// Damage +2    50     2       0
// Damage +3   100     3       0
// Defense +1   20     0       1
// Defense +2   40     0       2
// Defense +3   80     0       3

const DAGGER: Stats = Stats {
    cost: 8,
    attack: 4,
    defense: 0,
};
const SHORTSWORD: Stats = Stats {
    cost: 10,
    attack: 5,
    defense: 0,
};
const WARHAMMER: Stats = Stats {
    cost: 25,
    attack: 6,
    defense: 0,
};
const LONGSWORD: Stats = Stats {
    cost: 40,
    attack: 7,
    defense: 0,
};
const GREATAXE: Stats = Stats {
    cost: 74,
    attack: 8,
    defense: 0,
};
static WEAPONS: &'static [Stats] = &[DAGGER, SHORTSWORD, WARHAMMER, LONGSWORD, GREATAXE];

const OPTIONAL: Stats = Stats {
    cost: 0,
    attack: 0,
    defense: 0,
};
const LEATHER: Stats = Stats {
    cost: 13,
    attack: 0,
    defense: 1,
};
const CHAINMAIL: Stats = Stats {
    cost: 31,
    attack: 0,
    defense: 2,
};
const SPLINTMAIL: Stats = Stats {
    cost: 53,
    attack: 0,
    defense: 3,
};
const BANDEDMAIL: Stats = Stats {
    cost: 75,
    attack: 0,
    defense: 4,
};
const PLATEMAIL: Stats = Stats {
    cost: 102,
    attack: 0,
    defense: 5,
};
static ARMOR: &'static [Stats] = &[
    OPTIONAL, LEATHER, CHAINMAIL, SPLINTMAIL, BANDEDMAIL, PLATEMAIL,
];

const DAMAGE1: Stats = Stats {
    cost: 25,
    attack: 1,
    defense: 0,
};
const DAMAGE2: Stats = Stats {
    cost: 50,
    attack: 2,
    defense: 0,
};
const DAMAGE3: Stats = Stats {
    cost: 100,
    attack: 3,
    defense: 0,
};
const DEFENSE1: Stats = Stats {
    cost: 20,
    attack: 0,
    defense: 1,
};
const DEFENSE2: Stats = Stats {
    cost: 40,
    attack: 0,
    defense: 2,
};
const DEFENSE3: Stats = Stats {
    cost: 80,
    attack: 0,
    defense: 3,
};
static RINGS: &'static [Stats] = &[
    OPTIONAL, OPTIONAL, DAMAGE1, DAMAGE2, DAMAGE3, DEFENSE1, DEFENSE2, DEFENSE3,
];
