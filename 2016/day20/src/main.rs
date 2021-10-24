use advent::input_store;
use itertools::{self, Itertools};

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(line: &str) -> Self {
        let parts: Vec<u32> = line.split("-").map(|i| i.parse::<u32>().unwrap()).collect();
        let start = parts.get(0).unwrap().clone();
        let end = parts.get(1).unwrap().clone();

        Self { start, end }
    }
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        let within = self.start <= other.start && other.start <= self.end
            || self.start <= other.end && other.end <= self.end
            || other.start <= self.start && self.start <= other.end
            || other.start <= self.end && self.end <= other.end;

        let adjacent = (self.start > u32::MIN && self.start - 1 == other.end)
            || (self.end < u32::MAX && self.end + 1 == other.start);

        return within || adjacent;
    }

    fn combine(self, other: Self) -> Self {
        Self {
            start: if self.start < other.start {
                self.start
            } else {
                other.start
            },
            end: if self.end > other.end {
                self.end
            } else {
                other.end
            },
        }
    }

    fn contained(&self) -> u32 {
        self.end + 1 - self.start
    }
}

#[derive(Debug)]
struct Firewall {
    blocked_ranges: Vec<Range>,
}

impl From<String> for Firewall {
    fn from(input: String) -> Self {
        let mut ranges = Vec::new();

        for line in input.trim().lines() {
            ranges.push(line.into());
        }

        let mut firewall = Self {
            blocked_ranges: vec![],
        };

        for range in ranges {
            firewall.add_range(range);
        }

        firewall
    }
}

impl Firewall {
    fn add_range(&mut self, range: Range) {
        let mut range = range;
        let mut out = Vec::new();

        for r in self.blocked_ranges.iter().cloned() {
            if r.overlaps(&range) {
                range = range.combine(r);
            } else {
                out.push(r)
            }
        }
        out.push(range);

        out.sort();
        self.blocked_ranges = out;
    }

    fn allowed(&self) -> Vec<Range> {
        let mut out = Vec::new();
        for (left, right) in self.blocked_ranges.iter().tuple_windows() {
            out.push(Range {
                start: left.end + 1,
                end: right.start - 1,
            });
        }

        out
    }
}

fn main() {
    let input = input_store::get_input(2016, 20);
    //     let input = r#"5-8
    // 0-2
    // 4-7"#.to_string();

    let firewall: Firewall = input.into();

    println!(
        "part 1 => {}",
        firewall.blocked_ranges.get(0).unwrap().end + 1
    );

    println!(
        "part 2 => {}",
        firewall
            .allowed()
            .iter()
            .map(|r| r.contained())
            .sum::<u32>()
    );
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
    #[case("0-5".into(), "4-7".into(), true)]
    #[case("0-5".into(), "2-3".into(), true)]
    #[case("0-5".into(), "5-7".into(), true)]
    #[case("0-5".into(), "6-7".into(), true)]
    #[case("0-5".into(), "7-8".into(), false)]
    #[case("4-7".into(), "0-5".into(), true)]
    #[case("2-3".into(), "0-5".into(), true)]
    #[case("5-7".into(), "0-5".into(), true)]
    #[case("6-7".into(), "0-5".into(), true)]
    #[case("7-8".into(), "0-5".into(), false)]
    #[trace]

    fn test_overlap(#[case] left: Range, #[case] right: Range, #[case] expected: bool) {
        assert_eq!(left.overlaps(&right), expected);
    }

    #[rstest]
    #[case("0-5".into(), "4-7".into(), "0-7".into())]
    #[case("4-7".into(), "0-5".into(), "0-7".into())]
    #[case("0-1".into(), "2-3".into(), "0-3".into())]
    #[trace]

    fn test_combine(#[case] left: Range, #[case] right: Range, #[case] expected: Range) {
        assert_eq!(left.combine(right), expected);
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
