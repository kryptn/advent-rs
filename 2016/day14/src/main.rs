use std::collections::HashMap;

use itertools::Itertools;

use advent::input_store;

fn digest(input: &str) -> String {
    let d = md5::compute(input);
    format!("{:x}", d)
}

fn stretched_digest(input: &str) -> String {
    let mut key = digest(input);
    for _ in 0..2016 {
        let d = md5::compute(key);
        key = format!("{:x}", d);
    }
    key
}

fn seq_digest(digest: &str, len: usize) -> Vec<char> {
    let out: Vec<char> = digest
        .chars()
        .dedup_with_count()
        .filter(|(count, _)| count >= &len)
        .map(|(_, ch)| ch)
        .collect();

    out
}

#[derive(Debug)]
struct Digester {
    seed: String,
    idx: i64,
    memory: HashMap<char, u64>,
    first: bool,
    stretch: bool,
}

impl Digester {
    fn new(seed: String) -> Self {
        let mut memory = HashMap::new();
        for c in "0123456789abcdef".chars() {
            memory.insert(c, 0);
        }

        Self {
            seed,
            idx: -1,
            memory,
            first: true,
            stretch: false,
        }
    }

    fn generate_input(&self, offset: i64) -> String {
        let idx = self.idx + offset;
        format!("{}{}", self.seed, idx)
    }

    fn inc(&mut self, digest: &str) {
        let digest_count = seq_digest(digest, 5);
        for c in digest_count {
            *self.memory.get_mut(&c).unwrap() += 1;
        }
    }

    fn dec(&mut self, digest: &str) {
        let digest_count = seq_digest(digest, 5);
        for c in digest_count {
            *self.memory.get_mut(&c).unwrap() -= 1;
        }
    }

    fn first_fill(&mut self) {
        for i in 0..1000 {
            let input = format!("{}{}", &self.seed, i);
            let digest = if self.stretch {
                stretched_digest(&input)
            } else {
                digest(&input)
            };

            self.inc(&digest);
        }
    }
}

impl Iterator for Digester {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first_fill();
            self.first = false;
        }

        loop {
            self.idx += 1;

            let (candidate, follower) = if self.stretch {
                let candidate = stretched_digest(&self.generate_input(0));
                let follower = stretched_digest(&self.generate_input(1000));
                (candidate, follower)
            } else {
                let candidate = digest(&self.generate_input(0));
                let follower = digest(&self.generate_input(1000));
                (candidate, follower)
            };

            self.dec(&candidate);
            self.inc(&follower);

            let first_triple = {
                let triples = seq_digest(&candidate, 3);
                if !triples.is_empty() {
                    Some(triples.first().unwrap().clone())
                } else {
                    None
                }
            };

            if let Some(c) = first_triple {
                if self.memory.get(&c).unwrap() > &0 {
                    return Some(candidate);
                }
            }
        }
    }
}

fn main() {
    let input = input_store::get_input(2016, 14);
    //let input = "abc";

    let mut digester = Digester::new(input.trim().to_string());

    for _ in 0..64 {
        digester.next();
    }
    println!("part 1 => {}", digester.idx);

    let mut digester = Digester::new(input.trim().to_string());
    digester.stretch = true;
    for _ in 0..64 {
        digester.next();
    }
    println!("part 2 => {}", digester.idx);
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
    #[case("aaabbb", 3, vec!['a', 'b'])]
    #[case("aaaabbb", 3, vec!['a', 'b'])]
    #[case("aaaabb", 3, vec!['a'])]
    #[case("aaaaabbbccccc", 5, vec!['a', 'c'])]
    #[trace]
    fn seq_digest_tests(#[case] input: &str, #[case] len: usize, #[case] expected: Vec<char>) {
        let result = seq_digest(input, len);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("abc0", "a107ff634856bb300138cac6568c0f24")]
    fn test_stretched_digest(#[case] input: &str, #[case] expected: &str) {
        let d = stretched_digest(input);
        assert_eq!(d, expected);
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
