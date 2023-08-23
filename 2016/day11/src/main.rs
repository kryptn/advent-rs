use core::hash::Hash;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    u32,
};

use advent::input_store;
use itertools::Itertools;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    floor: u32,
    floors: u32,
    items: HashMap<u32, u32>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.floor.hash(state);
        let items: Vec<(u32, u32)> = self.items.iter().map(|(&i, &f)| (i, f)).sorted().collect();
        items.hash(state);
    }
}

impl State {
    fn from_slice(items: &[u32], floors: u32) -> Self {
        let mut out: Self = Default::default();
        out.floor = 1;
        out.floors = floors;
        out.items = items
            .into_iter()
            .enumerate()
            .map(|(idx, &floor)| (2u32.pow(idx as u32), floor))
            .collect();

        out
    }

    fn available_items(&self) -> Vec<u32> {
        self.items
            .iter()
            .filter_map(|(&i, &f)| if f == self.floor { Some(i) } else { None })
            .collect()
    }

    fn available_directions(&self) -> Vec<u32> {
        let mut out = vec![];

        if self.floor > 1 {
            out.push(self.floor - 1);
        }
        if self.floor < self.floors {
            out.push(self.floor + 1)
        }

        out
    }

    fn branches(&self) -> Vec<State> {
        let mut candidates = vec![];

        for next_floor in self.available_directions() {
            let items = self.available_items();
            let mut combinations: Vec<_> = items.iter().cloned().map(|i| vec![i]).collect();
            combinations.extend(items.into_iter().combinations(2));
            for items in combinations {
                let mut next = self.clone();
                next.floor = next_floor;
                for item in items {
                    next.items.entry(item).and_modify(|f| *f = next_floor);
                }
                if next.valid() {
                    candidates.push(next);
                }
            }
        }
        candidates.sort_by_key(Self::score);
        candidates
    }

    fn valid(&self) -> bool {
        // println!("\n\n\n\n");
        let mut items: HashMap<&u32, u32> = HashMap::new();

        for (item, floor) in &self.items {
            *items.entry(floor).or_default() += item;
        }

        for (floor, items) in items {
            let mask: u32 = 0b10101010101010101010101010101010;
            let chips = (items & mask) >> 1;
            let gens = items & (mask >> 1);
            let unpaired = chips ^ gens;
            let unpaired_chips = chips & unpaired > 0;

            // println!("floor {}, items: {:#08b}\n", floor, items);
            // println!("gens:           {:#08b}", gens);
            // println!("chips:          {:#08b}", chips);
            // println!("unpaired:       {:#08b}", unpaired);
            // println!("unpaired chips: {:#08b}", chips & unpaired);

            let valid = match (unpaired_chips, gens > 0) {
                (true, true) => false,
                _ => true,
            };

            // println!("valid: {}\n\n", valid);
            if !valid {
                return false;
            }
        }
        true
    }

    fn score(&self) -> u32 {
        self.items.values().map(|f| self.floors - f).sum()
    }

    fn complete(&self) -> bool {
        self.score() == 0
    }

    fn find(&self) -> u32 {
        let mut steps = 0;

        let mut layer = vec![self.clone()];
        let mut cache: HashSet<State> = layer.clone().into_iter().collect();

        loop {
            steps += 1;
            let next_layer: Vec<_> = layer
                .iter()
                .map(|l| l.branches())
                .flatten()
                .filter(|s| !cache.contains(s))
                .sorted_by_key(Self::score)
                // .take(50000)
                .collect();

            cache.extend(next_layer.clone());

            layer = next_layer;

            println!(
                "step {}, layer len: {}, cache size: {}",
                steps,
                layer.len(),
                cache.len()
            );
            if layer[0].complete() {
                break;
            }
        }
        steps
    }
}

fn main() {
    let input = input_store::get_input(2016, 11);

    // &[genA floor, chipA floor, genB floor, chipB floor, ...]

    let input = &[1, 1, 2, 3, 2, 3, 2, 3, 2, 3];
    // let input = &[2, 1, 3, 1];

    let mut problem = State::from_slice(input, 4);

    let mut cache: HashSet<State> = vec![problem.clone()].into_iter().collect();
    dbg!(cache.contains(&problem));

    let part_1 = problem.find();
    println!("part 1 -> {:?}", part_1);

    // dbg!(problem.available_directions());
    // dbg!(problem.available_items());
    // dbg!(problem.branches());
    // dbg!(&problem);
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
