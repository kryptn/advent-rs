use advent::input_store;
use advent_toolbox::range::{Range, Ranges};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const YEAR: usize = 2023;
const DAY: usize = 5;

#[derive(Debug)]
struct Transformer {
    destination: isize,
    source: isize,
    range_length: isize,
}

impl Transformer {
    fn apply(&self, input: isize) -> Option<isize> {
        if input >= self.source && input < self.source + self.range_length {
            Some(self.destination + (input - self.source))
        } else {
            None
        }
    }

    fn range(&self) -> Range {
        Range(self.source, self.source + self.range_length)
    }

    fn apply_range(&self, input: Range) -> Option<Vec<Range>> {
        let this_range = Range(self.source, self.source + self.range_length);
        if !input.intersects(&this_range) {
            return None;
        }
        let ranges = input.separate(&this_range);
        let out = ranges
            .into_iter()
            .map(|r| {
                if this_range.contains(&r) {
                    let offset = self.destination - self.source;
                    r + offset
                } else {
                    r
                }
            })
            .collect();
        Some(out)
    }

    // fn blend(&self, input: Vec<Range>) -> Vec<Range> {
    //     let mut ranges = input;

    //     ranges.iter().map(|r| {

    //         for t in self.transformations.iter() {
    //             if !t.range().intersects(r) {
    //                 continue;
    //             }
    //             let mut new_ranges = vec![];
    //             for r in ranges.iter() {
    //                 if !t.range().intersects(r) {
    //                     new_ranges.push(*r);
    //                     continue;
    //                 }
    //                 new_ranges.extend(t.range().separate(r));
    //             }
    //             ranges = new_ranges;
    //         }
    //     })

    //     ranges
    // }
}

impl From<String> for Transformer {
    fn from(input: String) -> Self {
        let parts = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            destination: parts[0],
            source: parts[1],
            range_length: parts[2],
        }
    }
}

#[derive(Debug)]
struct Map {
    transformations: Vec<Transformer>,
}

impl Map {
    fn transform(&self, input: isize) -> isize {
        // assume there's just one valid?
        self.transformations
            .iter()
            .find_map(|t| t.apply(input))
            .unwrap_or_else(|| input)
    }

    fn transform_ranges(&self, input: Vec<Range>) -> Vec<Range> {
        for t in self.transformations.iter() {
            let mut applied = false;
            for range in input.iter() {
                if !t.range().intersects(range) {
                    continue;
                }
                applied = true;
                break;
            }
            if !applied {
                return input;
            }
        }

        let mut split_up = vec![];
        for range in input.iter() {
            let mut applied = false;
            for t in self.transformations.iter() {
                if !t.range().intersects(range) {
                    continue;
                }
                split_up.extend(t.range().separate(range));

                // if let Some(ranges) = t.apply_range(*range) {
                //     split_up.extend(ranges);
                // } else {
                //     split_up.push(*range);
                // }
            }
            if !applied {
                split_up.push(*range);
            }
        }

        dbg!(split_up);

        self.transformations
            .iter()
            .cartesian_product(input.iter())
            .filter_map(|(t, r)| t.apply_range(*r))
            .flatten()
            .collect::<Vec<_>>()
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

fn seeds_from(line: &str) -> Vec<isize> {
    let parts = line.trim().split(": ").collect::<Vec<_>>();
    parts[1]
        .split(" ")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"seeds: 79 14 55 13

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

    let part_1_seeds: Vec<Range> = seeds
        .iter()
        .map(|s| {
            let s = *s as isize;
            Range::from((s, s + 1))
        })
        .sorted()
        .collect();
    let mut part_1 = part_1_seeds.clone();
    dbg!(&part_1);

    for map in maps.iter() {
        part_1 = map.transform_ranges(part_1);
        part_1 = Ranges(part_1).coalesce().0;
        dbg!(&part_1, map);
        break;
    }
    part_1.sort();

    println!("part_1 => {}", part_1[0].0);

    let actual_seeds: Vec<isize> = seeds
        .chunks(2)
        .map(|ch| (ch[0]..ch[0] + ch[1]))
        .flatten()
        .collect();

    dbg!(actual_seeds.len());
    // let locations = actual_seeds
    //     .par_iter()
    //     .map(|s| {
    //         let mut value = *s;
    //         for map in maps.iter() {
    //             value = map.transform(value);
    //         }
    //         value
    //     })
    //     .min();
    // println!("part_2 => {}", locations.unwrap());
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
