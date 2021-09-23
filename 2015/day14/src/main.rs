use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    pin::Pin,
    str::FromStr,
};

use advent::fetch;
use anyhow;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
struct Reindeer {
    name: String,
    velocity: i32,
    velocity_timespan: i32,
    rest_timespan: i32,
}

impl Reindeer {
    fn distance(&self, seconds: i32) -> i32 {
        let cycle_length = self.velocity_timespan + self.rest_timespan;

        let cycles = seconds / cycle_length;
        let remaining = seconds - (cycle_length * cycles);

        if remaining > self.velocity_timespan {
            // would be resting
            let cycles = cycles + 1;

            let out = self.velocity * self.velocity_timespan * cycles;
            out
        } else {
            let out = self.velocity * self.velocity_timespan * cycles + self.velocity * remaining;
            out
        }
    }

    fn distance_tuple(&self, seconds: i32) -> (String, i32) {
        (self.name.clone(), self.distance(seconds))
    }
}

fn parse_reindeer(line: &str) -> Reindeer {
    let parts: Vec<&str> = line.split(" ").collect_vec();

    Reindeer {
        name: parts[0].to_string(),
        velocity: parts[3].parse::<i32>().unwrap(),
        velocity_timespan: parts[6].parse::<i32>().unwrap(),
        rest_timespan: parts[13].parse::<i32>().unwrap(),
    }
}

const SECONDS: i32 = 2503;
//const SECONDS: i32 = 1000;

fn main() {
    let input = fetch::get_input(2015, 14);

    //     let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    // Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#.trim();

    let reindeer: Vec<Reindeer> = input.lines().map(|line| parse_reindeer(line)).collect();

    let max_dist = reindeer.iter().map(|r| r.distance(SECONDS)).max().unwrap();

    println!("part 1 => {}", max_dist);

    let mut fast_map: HashMap<String, i32> = HashMap::new();

    for i in 1..SECONDS {
        let (fastest, _) = reindeer
            .iter()
            .map(|r| r.distance_tuple(i))
            .reduce(|a, b| if a.1 > b.1 { a } else { b })
            .unwrap();
        if !fast_map.contains_key(&fastest) {
            fast_map.insert(fastest.clone(), 0);
        }
        *fast_map.get_mut(&fastest).unwrap() += 1;
    }

    dbg!(fast_map);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_parse() {
        let expected = Reindeer {
            name: "Vixen".to_string(),
            velocity: 19,
            velocity_timespan: 7,
            rest_timespan: 124,
        };
        let line = "Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.";

        assert_eq!(parse_reindeer(line), expected);
    }

    #[test]
    fn test_distance() {
        let comet = parse_reindeer(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        );
        assert_eq!(comet.distance(1000), 1120);

        let comet = parse_reindeer(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        assert_eq!(comet.distance(1000), 1056);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
