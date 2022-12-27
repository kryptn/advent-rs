use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub},
};

use advent::input_store;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Default)]
struct Inventory {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geode: isize,
}

impl Inventory {
    fn cost_for(&self, material: Material) -> Self {
        let mut cost = Self::default();
        match material {
            Material::Ore => cost.ore = self.ore,
            Material::Clay => cost.clay = self.clay,
            Material::Obsidian => cost.obsidian = self.obsidian,
            Material::Geode => cost.geode = self.geode,
        }
        cost
    }

    fn invalid(&self) -> bool {
        self.ore < 0 || self.clay < 0 || self.obsidian < 0 || self.geode < 0
    }
}

impl AddAssign<Inventory> for Inventory {
    fn add_assign(&mut self, rhs: Inventory) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Sub<Inventory> for Inventory {
    type Output = Inventory;

    fn sub(self, rhs: Inventory) -> Self::Output {
        let ore = self.ore - rhs.ore;
        let clay = self.clay - rhs.clay;
        let obsidian = self.obsidian - rhs.obsidian;
        let geode = self.geode - rhs.geode;

        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl From<Material> for Inventory {
    fn from(mat: Material) -> Self {
        let mut out = Self::default();
        match mat {
            Material::Ore => out.ore += 1,
            Material::Clay => out.clay += 1,
            Material::Obsidian => out.obsidian += 1,
            Material::Geode => out.geode += 1,
        }
        out
    }
}

#[derive(Debug, Clone)]
struct State {
    blueprint: Blueprint,
    inventory: Inventory,
    robots: Inventory,
    minute: usize,
}

fn actions() -> Vec<Option<Material>> {
    vec![
        Some(Material::Geode),
        Some(Material::Obsidian),
        Some(Material::Clay),
        Some(Material::Ore),
        None,
    ]
}

impl State {
    fn new(blueprint: Blueprint) -> Self {
        let inventory = Inventory::default();
        let robots = Material::Ore.into();

        Self {
            blueprint,
            inventory,
            robots,
            minute: 0,
        }
    }

    fn apply(&self, action: Option<Material>) -> Option<Self> {
        let mut n = self.clone();
        n.minute += 1;

        // if n.minute == 3 {
        //     println!("minute: {}", n.minute);
        // }

        let pending = match action {
            Some(mat) => {
                n.inventory = n.inventory - self.blueprint.get(mat).cost.clone();
                mat.into()
            }
            None => Inventory::default(),
        };

        if n.inventory.invalid() {
            None
        } else {
            n.inventory += self.robots.clone();
            n.robots += pending;

            Some(n)
        }
    }

    fn max_geodes(&self, minutes: usize) -> isize {
        if self.minute >= minutes {
            return self.inventory.geode;
        }

        actions()
            .par_iter()
            .filter_map(|&act| self.apply(act))
            .map(|ns| ns.max_geodes(minutes))
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Robot {
    kind: Material,
    cost: Inventory,
}

impl Robot {
    fn new(kind: Material, ore: isize, clay: isize, obsidian: isize, geode: isize) -> Self {
        let cost = Inventory {
            ore,
            clay,
            obsidian,
            geode,
        };

        Self { kind, cost }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

impl Blueprint {
    fn get(&self, kind: Material) -> &Robot {
        match kind {
            Material::Ore => &self.ore,
            Material::Clay => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode => &self.geode,
        }
    }
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        let split: Vec<_> = input.trim().split_whitespace().collect();

        let get = |n: usize| -> isize { split[n].parse().unwrap() };

        let ore = Robot::new(Material::Ore, get(6), 0, 0, 0);
        let clay = Robot::new(Material::Clay, get(6), 0, 0, 0);
        let obsidian = Robot::new(Material::Obsidian, get(18), get(21), 0, 0);
        let geode = Robot::new(Material::Geode, get(27), 0, get(30), 0);

        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 19);
    let input = r#"Blueprint 1:    Each ore robot costs 4 ore.    Each clay robot costs 2 ore.    Each obsidian robot costs 3 ore and 14 clay.    Each geode robot costs 2 ore and 7 obsidian.
  Blueprint 2:    Each ore robot costs 2 ore.    Each clay robot costs 3 ore.    Each obsidian robot costs 3 ore and 8 clay.    Each geode robot costs 3 ore and 12 obsidian."#;

    let blueprints: Vec<Blueprint> = input.trim().lines().map(|line| line.into()).collect();

    let quality: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1, State::new(b.clone())))
        .map(|(i, state)| (i, state.max_geodes(24)))
        .sorted_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .inspect(|x| {
            dbg!(x);
        })
        .map(|(i, m)| i * m as usize)
        .sum();

    println!("part_1 => {}", quality);
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
