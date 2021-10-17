use advent::input_store;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Disc {
    offset: u64,
    position: u64,
    total: u64,
}

impl From<&str> for Disc {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(" ").collect();
        let offset = parts
            .get(1)
            .unwrap()
            .clone()
            .strip_prefix("#")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let total = parts.get(3).unwrap().parse::<u64>().unwrap();
        let position = parts
            .get(11)
            .unwrap()
            .clone()
            .strip_suffix(".")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        Self {
            offset,
            position,
            total,
        }
    }
}

impl Disc {
    fn new(position: u64, total: u64, offset: u64) -> Self {
        Self {
            offset,
            position,
            total,
        }
    }

    fn open_at(&self, n: u64) -> bool {
        let rv = (n + self.offset + self.position) % self.total;
        // println!(
        //     "({} + {} + {}) % {} == {}",
        //     n, self.offset, self.position, self.total, rv
        // );
        rv == 0
    }
}

struct Discs {
    discs: Vec<Disc>,
    step: u64,
    time: u64,
}

impl Discs {
    fn falls_through(&self, at: u64) -> bool {
        self.discs.iter().map(|d| d.open_at(at)).all(|v| v)
    }

    fn next_step(&self) -> u64 {
        let v = self
            .discs
            .iter()
            .filter(|&d| d.open_at(self.time))
            .map(|d| d.total)
            .reduce(|a, b| a * b);
        match v {
            Some(step) => step,
            None => self.step,
        }
    }
}

impl From<Vec<Disc>> for Discs {
    fn from(discs: Vec<Disc>) -> Self {
        Self {
            discs,
            step: 1,
            time: 0,
        }
    }
}

impl Iterator for Discs {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.falls_through(self.time) {
                return Some(self.time);
            } else {
                self.step = self.next_step();
                self.time += self.step;
            }
        }
    }
}

fn main() {
    let input = input_store::get_input(2016, 15);
    //     let input = r#"Disc #1 has 5 positions; at time=0, it is at position 4.
    // Disc #2 has 2 positions; at time=0, it is at position 1."#;

    let mut discs: Vec<Disc> = input.lines().map(|l| Disc::from(l)).collect();
    let mut part_1: Discs = discs.clone().into();

    println!("part 1 => {:?}", part_1.next());

    discs.push(Disc {
        offset: discs.len() as u64 + 1,
        total: 11,
        position: 0,
    });
    let mut part_2: Discs = discs.clone().into();
    println!("part 1 => {:?}", part_2.next());
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
    #[case(1, 5, 4, 0, true)]
    #[case(1, 5, 4, 5, true)]
    #[case(2, 2, 1, 0, false)]
    #[case(2, 2, 1, 5, true)]
    #[trace]
    fn test_disc_open_at(
        #[case] offset: u64,
        #[case] total: u64,
        #[case] position: u64,
        #[case] press_at: u64,
        #[case] expected: bool,
    ) {
        let d = Disc::new(position, total, offset);
        assert_eq!(d.open_at(press_at), expected);
    }

    #[test]
    fn test_parse() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.";
        let expected = Disc {
            offset: 1,
            total: 5,
            position: 4,
        };

        let disc: Disc = input.into();
        assert_eq!(disc, expected);
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
