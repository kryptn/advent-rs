use core::hash::Hash;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    u32,
};

use advent::input_store;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    floor: u32,
    floors: u32,

    // item -> floor
    items: HashMap<u32, u32>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.floor.hash(state);
        let items: Vec<(u32, u32)> = self.items.iter().map(|(&i, &f)| (i, f)).sorted().collect();
        items.hash(state);
    }
}

lazy_static::lazy_static! {
    static ref LOOKUP_MAP: HashMap<u32, &'static str> = {
        let mut map = HashMap::new();
        let items = ["AM", "AG", "BM", "BG", "CM", "CG", "DM", "DG", "EM", "EG"];
        for (idx, item) in items.iter().enumerate() {
            map.insert(2u32.pow(idx as u32), *item);
        }
        map
    };
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<(u32, u32)> =
            self.items.iter().map(|(&i, &f)| (i, f)).sorted().collect();
        items.reverse();
        for floor in (1..=self.floors).rev() {
            write!(f, "F{} ", floor)?;
            if self.floor == floor {
                write!(f, "E ")?;
            } else {
                write!(f, "  ")?;
            }
            for (item, item_floor) in &items {
                if *item_floor == floor {
                    write!(f, "{} ", LOOKUP_MAP[&item])?;
                } else {
                    write!(f, ".. ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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

        // possible items to take with us
        let items = self.available_items();
        let mut combinations: Vec<_> = items.iter().cloned().map(|i| vec![i]).collect();
        combinations.extend(items.into_iter().combinations(2));

        let mut valid_this_layer = 0;
        let mut invalid_this_layer = 0;

        for next_floor in self.available_directions() {
            for items in &combinations {
                let mut next = self.clone();
                next.floor = next_floor;
                for item in items {
                    next.items.entry(*item).and_modify(|f| *f = next_floor);
                }
                if next.valid() {
                    candidates.push(next);
                    valid_this_layer += 1;
                } else {
                    invalid_this_layer += 1;
                }
            }
        }

        // candidates.sort_by_key(Self::score);
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

            if unpaired_chips && gens > 0 {
                return false;
            }
        }
        true
    }

    fn score(&self) -> usize {
        self.items
            .values()
            .map(|f| (self.floors - f) as usize)
            .sum()
    }

    fn complete(&self) -> bool {
        self.score() == 0
    }

    fn find(&self) -> u32 {
        let mut steps = 0;

        let mut layer = vec![self.clone()];
        let mut cache: HashSet<State> = layer.clone().into_iter().collect();
        // let mut valid_this_layer = 0;
        // let mut invalid_this_layer = 0;
        loop {
            steps += 1;
            let mut next_layer: Vec<_> = layer
                .par_iter()
                .map(|l| l.branches())
                .flatten()
                .filter(|s| !cache.contains(s))
                // .take(50000)
                .collect();

            next_layer.sort_by_key(Self::score);

            cache.extend(next_layer.clone());

            let first = next_layer[0].clone();
            let last = next_layer.last().unwrap().clone();

            // println!("valid: {}, invalid: {}", valid_this_layer, invalid_this_layer);
            println!(
                "first score: {}, last score: {}",
                first.score(),
                last.score()
            );

            // layer = next_layer.into_iter().take(5000000).collect();

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
            // valid_this_layer = 0;
            // invalid_this_layer = 0;
        }
        steps
    }

    fn dfs(&self, path: Vec<State>, cache: &Cache) -> Vec<State> {
        println!("path len: {}\n\n{}", path.len(), self);

        {
            let mut cache_guard = cache.cache.borrow_mut();
            if cache_guard.contains(self) {
                return vec![];
            }
            cache_guard.insert(self.clone());
        }
        if path.len() > *cache.lowest_score.borrow() {
            return vec![];
        }

        let mut path = path;

        // dbg!(path.len(), self.score());
        path.push(self.clone());

        if self.complete() {
            let mut cg = cache.results.borrow_mut();
            cg.push(path.clone());
            let mut ls_g = cache.lowest_score.borrow_mut();
            if path.len() < *ls_g {
                *ls_g = path.len();
                println!("new lowest score: {}", ls_g);
            }
            // return path;
            return vec![];
        }

        let mut branches = self.branches();
        branches.sort_by_key(Self::score);

        for branch in branches {
            let path = path.clone();
            let result = branch.dfs(path, cache);
            if result.len() > 0 {
                return result;
            }
        }

        vec![]
    }
}

#[derive(Default)]
struct Cache {
    cache: RefCell<HashSet<State>>,
    results: RefCell<Vec<Vec<State>>>,
    lowest_score: RefCell<usize>,
}

fn main() {
    let input = input_store::get_input(2016, 11);

    // &[genA floor, chipA floor, genB floor, chipB floor, ...]

    let input = &[1, 1, 2, 3, 2, 3, 2, 3, 2, 3];
    // let input = &[2, 1, 3, 1];

    let mut problem = State::from_slice(input, 4);

    println!("problem: \n\n{}", problem);
    // return;

    let mut cache: HashSet<State> = vec![problem.clone()].into_iter().collect();
    // dbg!(cache.contains(&problem));

    // let part_1 = problem.find();

    let cache = Cache::default();
    cache.lowest_score.replace(u32::MAX as usize);
    let part_1 = problem.dfs(vec![], &cache);

    let cr_g = cache.results.borrow();
    let shortest = cr_g.iter().min_by_key(|p| p.len()).unwrap();
    println!("part 1 -> {:?}", shortest.len() - 1);

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
