use advent::input_store;

#[derive(Clone)]
struct Elf {
    next: usize,
    count: u32,
}

struct Party {
    elves: Vec<Elf>,
    current: usize,
}

impl Party {
    fn next_with_presents(&self, from: usize) -> usize {
        for i in 1..self.elves.len() {
            let elf = self.elves.get(from + i).expect("limiting index");
            if elf.count > 0 {
                return from + i;
            }
        }
        unreachable!();
    }
}

impl From<String> for Party {
    fn from(k: String) -> Self {
        let k = k.parse().unwrap();
        let mut elves = Vec::new();
        for i in 0..k {
            elves.push(Elf {
                next: (i + 1) % k,
                count: 1,
            })
        }

        Self { elves, current: 0 }
    }
}

impl Iterator for Party {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let target_index = { self.elves.get(self.current).unwrap().next };

        if target_index == self.current {
            return None;
        }

        let target = self.elves.get(target_index).unwrap().clone();
        let this = self.elves.get_mut(self.current).unwrap();

        this.count += target.count;
        this.next = target.next;
        self.current = this.next;

        Some(self.current + 1)
    }
}

fn main() {
    let input = input_store::get_input(2016, 19).trim().to_string();

    let party: Party = input.into();
    let winner = party.last();
    println!("part 1 => {:?}", winner);
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
        let mut party: Party = String::from("5").into();

        let out = party.last();

        assert_eq!(out, Some(3))
    }


    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
