use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    ops::Index,
    str::FromStr,
};

use advent::fetch;
use anyhow;
use itertools::Itertools;

struct LetterIter {
    curr: String,
}

const A: u8 = 97;
const Z: u8 = 122;

impl Iterator for LetterIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe { self.curr.as_bytes_mut() };
        for i in 0..current.len() {
            let idx = current.len() - 1 - i;
            let sub = current[idx] + 1;

            if sub > Z {
                current[idx] = A;
            } else {
                current[idx] = sub;
                break;
            }
        }

        let out = std::str::from_utf8(current).unwrap();
        Some(out.to_string())

        // if current[0] == Z && current.to_vec().iter().all_equal() {
        //     None
        // } else {

        // }
    }
}

struct Password {
    curr: LetterIter,
}

impl Iterator for Password {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pw = self.curr.next()?;
            if valid_pw(&pw) {
                return Some(pw);
            }
        }
    }
}

fn valid_pw(pw: &str) -> bool {
    if pw.contains('i') || pw.contains('o') || pw.contains('l') {
        return false;
    }

    let mut found = false;
    for (x, y, z) in pw.bytes().tuple_windows() {
        if y == x + 1 && z == x + 2 {
            found = true;
            break;
        }
    }
    if !found {
        return false;
    }

    let mut pairs: u8 = 0;
    let mut skip = false;
    let pwb = pw.as_bytes();
    for i in 1..pwb.len() {
        if skip {
            skip = false;
            continue;
        }
        if pwb[i] == pwb[i - 1] {
            skip = true;
            pairs += 1;
        }
    }

    if pairs < 2 {
        return false;
    }

    true
}

fn main() {
    let input = fetch::get_input(2015, 11);

    let mut pw_iter = Password {
        curr: LetterIter { curr: input },
    };

    println!("part 1 => {}", pw_iter.next().unwrap());
    println!("part 2 => {}", pw_iter.next().unwrap());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn str_iter_tests() {
        let mut li = LetterIter {
            curr: String::from("ay"),
        };

        assert_eq!(li.next(), Some(String::from("az")));
        assert_eq!(li.next(), Some(String::from("ba")));

        let mut li = LetterIter {
            curr: String::from("zy"),
        };

        assert_eq!(li.next(), Some(String::from("zz")));
        assert_eq!(li.next(), Some(String::from("aa")));
    }

    #[test]
    fn validator_tests() {
        assert_eq!(valid_pw("hijklmmn"), false);
        assert_eq!(valid_pw("hij"), false);
        assert_eq!(valid_pw("abbceffg"), false);
        assert_eq!(valid_pw("abbcegjk"), false);
    }

    #[test]
    fn p1_tests() {
        let mut pw_iter = Password {
            curr: LetterIter {
                curr: String::from("abcdefgh"),
            },
        };
        assert_eq!(pw_iter.next(), Some(String::from("abcdffaa")));

        let mut pw_iter = Password {
            curr: LetterIter {
                curr: String::from("ghijklmn"),
            },
        };
        assert_eq!(pw_iter.next(), Some(String::from("ghjaabcc")));
    }

    #[test]
    fn p2_tests() {}
}
