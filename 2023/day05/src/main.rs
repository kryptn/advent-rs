use advent::input_store;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const YEAR: usize = 2023;
const DAY: usize = 5;

struct Transformer {
    destination: usize,
    source: usize,
    range_length: usize,
}

impl Transformer {
    fn apply(&self, input: usize) -> Option<usize> {
        if input >= self.source && input < self.source + self.range_length {
            Some(self.destination + (input - self.source))
        } else {
            None
        }
    }
}

impl From<String> for Transformer {
    fn from(input: String) -> Self {
        let parts = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            destination: parts[0],
            source: parts[1],
            range_length: parts[2],
        }
    }
}

struct Map {
    transformations: Vec<Transformer>,
}

impl Map {
    fn transform(&self, input: usize) -> usize {
        // assume there's just one valid?
        self.transformations
            .iter()
            .find_map(|t| t.apply(input))
            .unwrap_or_else(|| input)
    }
}

impl From<String> for Map {
    fn from(input: String) -> Self {
        let lines = input.trim().lines().skip(1);
        Self {
            transformations: lines.map(|line| line.trim().to_string().into()).collect(),
        }
    }
}

fn seeds_from(line: &str) -> Vec<usize> {
    let parts = line.trim().split(": ").collect::<Vec<_>>();
    parts[1]
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // humidity-to-location map:
    // 60 56 37
    // 56 93 4"#;

    let sections = input.trim().split("\n\n").collect::<Vec<_>>();

    let seeds = seeds_from(sections[0]);

    let maps = sections[1..]
        .iter()
        .map(|s| Map::from(s.to_string()))
        .collect::<Vec<_>>();

    let locations = seeds
        .iter()
        .map(|s| {
            let mut value = *s;
            for map in maps.iter() {
                value = map.transform(value);
            }
            value
        })
        .min();
    println!("part_1 => {}", locations.unwrap());

    let actual_seeds: Vec<usize> = seeds
        .chunks(2)
        .map(|ch| (ch[0]..ch[0] + ch[1]))
        .flatten()
        .collect();
    let locations = actual_seeds
        .par_iter()
        .map(|s| {
            let mut value = *s;
            for map in maps.iter() {
                value = map.transform(value);
            }
            value
        })
        .min();
    println!("part_2 => {}", locations.unwrap());
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
