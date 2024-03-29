use std::{cell::RefCell, sync::Arc};

use rayon::prelude::*;

use advent::fetch;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Effect {
    Shield(u8),
    Poison(u8),
    Recharge(u8),
}

enum BattleStatus {
    Culled(Battle),
    Win(Battle),
    Lose(Battle),
    Continue(Battle),
}

#[derive(Clone, Copy, Debug)]
struct Spell {
    mana_cost: i32,
    heal: i32,
    damage: i32,
    effect: Option<Effect>,
}

static MagicMissile: Spell = Spell {
    mana_cost: 53,
    heal: 0,
    damage: 4,
    effect: None,
};
static Drain: Spell = Spell {
    mana_cost: 73,
    heal: 2,
    damage: 2,
    effect: None,
};
static Shield: Spell = Spell {
    mana_cost: 113,
    heal: 0,
    damage: 0,
    effect: Some(Effect::Shield(6)),
};
static Poison: Spell = Spell {
    mana_cost: 173,
    heal: 0,
    damage: 0,
    effect: Some(Effect::Poison(6)),
};
static Recharge: Spell = Spell {
    mana_cost: 229,
    heal: 0,
    damage: 0,
    effect: Some(Effect::Recharge(5)),
};
static BossAttack: Spell = Spell {
    mana_cost: 0,
    heal: 0,
    damage: 9,
    effect: None,
};

static SPELLS: [&Spell; 5] = [&MagicMissile, &Drain, &Shield, &Poison, &Recharge];

#[derive(Clone, Debug, Default)]
struct Battle {
    player_health: i32,
    player_mana: i32,
    player_mana_spent: i32,
    boss_health: i32,
    boss_attack: i32,

    shield_turns: u8,
    poison_turns: u8,
    recharge_turns: u8,

    hard_mode: bool,
    hurestic: Arc<RefCell<i32>>,
    actions: Vec<&'static Spell>,
}

impl Battle {
    fn branches(&self) -> Vec<BattleStatus> {
        let mut temp = self.with_effects();
        if self.hard_mode {
            temp.player_health -= 1;
        }

        let status = temp.status();
        let temp = match status {
            BattleStatus::Culled(_) | BattleStatus::Win(_) | BattleStatus::Lose(_) => {
                return vec![status]
            }
            BattleStatus::Continue(battle) => battle,
        };

        // valid spells
        let mut out = vec![];
        for spell in SPELLS {
            if temp.player_mana < spell.mana_cost {
                continue;
            }

            if let Some(effect) = spell.effect {
                if match effect {
                    Effect::Shield(_) => temp.shield_turns > 0,
                    Effect::Poison(_) => temp.poison_turns > 0,
                    Effect::Recharge(_) => temp.recharge_turns > 0,
                } {
                    continue;
                }
            }

            out.push(spell)
        }

        // if no spells, lose
        if out.is_empty() {
            return vec![BattleStatus::Lose(temp)];
        }

        out.into_iter().map(|spell| temp.with(&spell)).collect()
    }

    fn bound(&self) -> bool {
        self.player_mana_spent < *self.hurestic.borrow()
    }

    fn status(self) -> BattleStatus {
        if self.boss_health <= 0 {
            // println!(
            //     "win! actions: {}, mana spent: {}",
            //     self.actions.len(),
            //     self.player_mana_spent
            // );
            if *self.hurestic.borrow() > self.player_mana_spent {
                let mut lowest = self.hurestic.borrow_mut();
                *lowest = self.player_mana_spent;
            }
            BattleStatus::Win(self)
        } else if self.player_health <= 0 {
            BattleStatus::Lose(self)
        } else if self.player_mana_spent > *self.hurestic.borrow() {
            BattleStatus::Continue(self)
        } else {
            BattleStatus::Continue(self)
        }
    }

    fn with_effects(&self) -> Self {
        let mut out = self.clone();
        if out.shield_turns > 0 {
            out.shield_turns -= 1;
        }
        if out.poison_turns > 0 {
            out.poison_turns -= 1;
            out.boss_health -= 3;
        }
        if out.recharge_turns > 0 {
            out.recharge_turns -= 1;
            out.player_mana += 101;
        }

        out
    }

    fn with(&self, spell: &'static Spell) -> BattleStatus {
        let mut out = self.clone();
        out.player_mana -= spell.mana_cost;
        out.player_mana_spent += spell.mana_cost;
        out.boss_health -= spell.damage;
        out.player_health += spell.heal;
        out.actions.push(spell);
        if let Some(effect) = spell.effect {
            match effect {
                Effect::Shield(turns) => out.shield_turns = turns,
                Effect::Poison(turns) => out.poison_turns = turns,
                Effect::Recharge(turns) => out.recharge_turns = turns,
            }
        }

        // player action done
        // check for end

        let status = out.status();
        out = match status {
            BattleStatus::Continue(battle) => battle,
            _ => return status,
        };

        out = out.with_effects();
        let status = out.status();
        out = match status {
            BattleStatus::Continue(battle) => battle,
            _ => return status,
        };

        out.player_health -= {
            let defense_mod = if out.shield_turns > 0 { 7 } else { 0 };
            let boss_damage = Ord::max(1, out.boss_attack - defense_mod);
            boss_damage
        };

        out.status()
    }

    fn next_layer(&self) -> impl Iterator<Item = Self> {
        self.branches().into_iter().filter_map(|bs| match bs {
            BattleStatus::Continue(b) => {
                if b.bound() {
                    Some(b)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn solve(&self) {
        let mut layer = vec![self.clone()];
        let mut iteration = 0;
        while layer.len() > 0 {
            // println!("loop {}, layer len: {}", iteration, layer.len());
            // println!(
            //     "    first boss health: {}, last boss health: {}",
            //     layer[0].boss_health,
            //     layer[layer.len() - 1].boss_health
            // );
            iteration += 1;
            let next_layer = layer
                .into_iter()
                .map(|b| b.next_layer())
                .flatten()
                .sorted_by(|a, b| a.boss_health.cmp(&b.boss_health))
                .collect();
            layer = next_layer;
        }
    }
}

fn main() {
    let _input = fetch::get_input(2015, 22);

    let lowest_mana = Arc::new(RefCell::new(i32::MAX));

    let mut battle = Battle::default();
    battle.player_health = 50;
    battle.player_mana = 500;
    battle.boss_attack = 9;
    battle.boss_health = 51;
    battle.hurestic = lowest_mana.clone();
    battle.solve();
    println!("part 1 -> {}", lowest_mana.borrow());

    *lowest_mana.borrow_mut() = i32::MAX;
    battle.hard_mode = true;
    battle.solve();
    println!("part 2 -> {}", lowest_mana.borrow());
}

#[cfg(test)]
mod test {

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
