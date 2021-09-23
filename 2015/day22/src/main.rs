use std::{cell::RefCell, rc::Rc, sync::Mutex};

use advent::{fetch, numbers::factors};
use itertools::{Combinations, Itertools, Powerset};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Effect {
    Shield(i32),
    Poison(i32),
    Recharge(i32),
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
    damage: 4,
    effect: Some(Effect::Poison(6)),
};
static Recharge: Spell = Spell {
    mana_cost: 229,
    heal: 0,
    damage: 4,
    effect: Some(Effect::Recharge(5)),
};

static Spells: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];

#[derive(Clone, Debug)]
struct Entity {
    health: i32,
    mana: Option<i32>,
    attack: Option<i32>,

    spent_mana: i32,

    active_effects: Vec<Effect>,
}

impl Entity {
    fn apply_effect(&mut self, other: &mut Self, effect: Effect, indent: i32) -> Option<Effect> {
        match effect {
            Effect::Shield(turns) => {
                println!("{}inner effect {:?}", indented("", indent), effect);
                if turns <= 1 {
                    None
                } else {
                    Some(Effect::Shield(turns - 1))
                }
            }
            Effect::Poison(turns) => {
                println!("{}inner effect {:?}", indented("", indent), effect);
                other.health -= 3;
                if turns <= 1 {
                    None
                } else {
                    Some(Effect::Poison(turns - 1))
                }
            }
            Effect::Recharge(turns) => {
                println!("{}inner effect {:?}", indented("", indent), effect);
                self.mana = Some(self.mana.unwrap() + 101);
                if turns <= 1 {
                    None
                } else {
                    Some(Effect::Recharge(turns - 1))
                }
            }
        }
    }

    fn apply_effects(&mut self, other: &mut Self, indent: i32) {
        let mut next_round = Vec::new();
        for effect in self.active_effects.clone() {
            let next_effect = self.apply_effect(other, effect, indent);
            if let Some(e) = next_effect {
                next_round.push(e);
            }
        }

        self.active_effects = next_round;
    }

    fn has_defense(&self) -> bool {
        for effect in &self.active_effects {
            if let Effect::Shield(_) = effect {
                return true;
            }
        }
        false
    }

    fn apply_damage(&mut self, damage: i32) {
        let defense = if self.has_defense() { 7 } else { 0 };
        let damage = {
            let d = damage - defense;
            if d <= 1 {
                1
            } else {
                d
            }
        };
        self.health -= damage;
    }

    fn valid_spells(&self, indent: i32) -> Vec<Spell> {
        let mut out = Vec::new();

        let mana = match self.mana {
            Some(m) => m,
            None => return out,
        };

        if mana > MagicMissile.mana_cost {
            out.push(MagicMissile);
        }
        if mana > Drain.mana_cost {
            out.push(Drain);
        }
        if mana > Shield.mana_cost
            && !self
                .active_effects
                .iter()
                .any(|e| matches!(e, Effect::Shield(_)))
        {
            out.push(Shield);
        }
        if mana > Poison.mana_cost
            && !self
                .active_effects
                .iter()
                .any(|e| matches!(e, Effect::Poison(_)))
        {
            out.push(Poison);
        }
        if mana > Recharge.mana_cost
            && !self
                .active_effects
                .iter()
                .any(|e| matches!(e, Effect::Recharge(_)))
        {
            out.push(Recharge);
        }

        println!("{}valid spells: {:?}", indented("", indent), out);

        out
    }

    fn use_spell(&mut self, other: &mut Entity, spell: Spell, indent: i32) {
        self.mana = Some(self.mana.unwrap() - spell.mana_cost);
        self.spent_mana += spell.mana_cost;
        self.health += spell.heal;
        other.health -= spell.damage;

        println!("{}player casts spell", indented("", indent));
        println!("{}{:?}", indented("", indent), self);
        println!("{}{:?}", indented("", indent), spell);

        if let Some(effect) = spell.effect {
            println!("{}applied effect {:?}", indented("", indent), effect);

            self.active_effects.push(effect);
        }
    }

    // fn step(&mut self, other: &mut Entity) {
    //     self.apply_effects(other);

    //     for spell in self.valid_spells() {
    //         self.use_spell(other, spell);
    //     }
    // }
}

#[derive(Clone, Debug)]
enum Turn {
    Player,
    Enemy,
}

impl Turn {
    fn other(&self) -> Self {
        match self {
            Turn::Player => Turn::Enemy,
            Turn::Enemy => Turn::Player,
        }
    }
}

#[derive(Clone, Debug)]
struct Battle {
    player: Entity,
    enemy: Entity,

    turn: Turn,

    lowest_mana_spent: Rc<RefCell<Option<i32>>>,
}

fn indented(msg: &str, indent: i32) -> String {
    let mut ind = String::from("");
    for _ in 0..indent {
        ind.push_str("    ");
    }

    ind.push_str(msg);

    ind
}

impl Battle {
    fn new(player: Entity, enemy: Entity) -> Self {
        let mana = Rc::new(RefCell::new(None));
        Self {
            player,
            enemy,
            turn: Turn::Player,
            lowest_mana_spent: mana,
        }
    }

    fn winner(&self, indent: i32) -> Option<Turn> {
        let lowest_spent = match self.lowest_mana_spent.borrow().clone() {
            Some(i) => i,
            None => 10000,
        };
        if self.player.valid_spells(indent).len() == 0 && self.player.health <= 0
            || self.player.spent_mana > lowest_spent
        {
            println!(
                "{}returning player lost {:?}",
                indented("", indent),
                self.player
            );
            println!(
                "{}                      {:?}",
                indented("", indent),
                self.enemy
            );
            println!(
                "{}                      lowest: {:?}",
                indented("", indent),
                self.lowest_mana_spent
            );

            Some(Turn::Enemy)
        } else if self.enemy.health <= 0 {
            println!(
                "{}returning player won {:?}",
                indented("", indent),
                self.player
            );
            println!(
                "{}                     {:?}",
                indented("", indent),
                self.enemy
            );
            println!(
                "{}                      lowest: {:?}",
                indented("", indent),
                self.lowest_mana_spent
            );
            Some(Turn::Player)
        } else {
            println!("{}no win  {:?}", indented("", indent), self.player);
            println!("{}        {:?}", indented("", indent), self.enemy);
            println!(
                "{}                      lowest: {:?}",
                indented("", indent),
                self.lowest_mana_spent
            );
            None
        }
    }

    fn apply_effects(&mut self, indent: i32) {
        self.player.apply_effects(&mut self.enemy, indent);
    }

    fn handle_turn(&mut self, indent: i32) -> Vec<Option<i32>> {
        println!("\n\n\n");
        self.apply_effects(indent);
        // let winner = self.winner();
        // dbg!(winner);
        match self.winner(indent) {
            Some(ent) => {
                return match ent {
                    Turn::Player => {
                        let lowest_spent = match self.lowest_mana_spent.borrow().clone() {
                            Some(i) => i,
                            None => 10000,
                        };
                        if self.player.spent_mana < lowest_spent {
                            let mut opt = self.lowest_mana_spent.borrow_mut();
                            opt.replace(self.player.spent_mana);
                        }
                        vec![Some(self.player.spent_mana)]
                    }
                    Turn::Enemy => vec![None],
                }
            }
            None => {}
        }

        let outcomes: Vec<Option<i32>> = match self.turn {
            Turn::Player => {
                //println!("matched player");
                //self.player.step(&mut self.enemy);
                let mut outcomes = Vec::new();

                for spell in self.player.valid_spells(indent) {
                    println!("\n{}turn{}", indented("", indent), indent);
                    println!("{}will cast {:?}", indented("", indent), spell);
                    let mut battle = self.clone();
                    battle.player.use_spell(&mut battle.enemy, spell, indent);
                    battle.turn = battle.turn.other();
                    outcomes.extend(battle.handle_turn(indent + 1));
                }
                outcomes
            }
            Turn::Enemy => {
                println!("\n{}turn{}", indented("", indent), indent);
                println!("{}enemy does damage ", indented("", indent));
                self.player.apply_damage(self.enemy.attack.unwrap());
                let mut battle = self.clone();
                battle.turn = self.turn.other();
                battle.handle_turn(indent + 1)
            }
        };

        //dbg!(&outcomes);

        outcomes
    }
}

fn main() {
    let input = fetch::get_input(2015, 22);

    let player = Entity {
        health: 50,
        mana: Some(500),
        spent_mana: 0,
        attack: None,
        active_effects: Vec::new(),
    };
    let enemy = Entity {
        health: 51,
        mana: None,
        spent_mana: 0,
        attack: Some(9),
        active_effects: Vec::new(),
    };
    // let player = Entity {
    //     health: 10,
    //     mana: Some(250),
    //     spent_mana: 0,
    //     attack: None,
    //     active_effects: Vec::new(),
    // };
    // let enemy = Entity {
    //     health: 14,
    //     mana: None,
    //     spent_mana: 0,
    //     attack: Some(8),
    //     active_effects: Vec::new(),
    // };

    let mut battle = Battle::new(player, enemy);

    let result = battle
        .handle_turn(0)
        .iter()
        .map(|v| v.unwrap_or(10000))
        .min();

    dbg!(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
