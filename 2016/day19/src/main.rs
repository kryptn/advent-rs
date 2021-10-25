use advent::input_store;

#[derive(Clone)]
struct Elf {
    index: usize,
    next: usize,
    count: u32,
}

struct Party {
    elves: Vec<Elf>,
    current: usize,
}

impl From<String> for Party {
    fn from(k: String) -> Self {
        let k = k.parse().unwrap();
        let mut elves = Vec::new();
        for i in 0..k {
            elves.push(Elf {
                next: (i + 1) % k,
                count: 1,
                index: i,
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

struct Party2 {
    elves: Vec<u32>,
}

impl From<String> for Party2 {
    fn from(k: String) -> Self {
        let k = k.parse().unwrap();
        let mut elves = Vec::new();
        for i in 0..k {
            elves.push(i + 1);
        }

        Self { elves }
    }
}

impl Iterator for Party2 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let target_index = self.elves.len() / 2;
        self.elves.remove(target_index);

        let this = self.elves.get(0)?.clone();
        self.elves.rotate_left(1);

        // if self.elves.len() % 1000 == 0 {
        //     println!("progress: {}",  self.elves.len());
        // }
        Some(this)
    }
}

fn main() {
    let input = input_store::get_input(2016, 19).trim().to_string();
    //let input = "5".to_string();

    let party: Party = input.clone().into();
    let winner = party.last();
    println!("part 1 => {:?}", winner);

    let party: Party2 = input.into();
    let winner = party.last();
    println!("part 2 => {:?}", winner);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {
        let party: Party = String::from("5").into();

        let out = party.last();

        assert_eq!(out, Some(3))
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
