use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    str::FromStr,
};

use advent::fetch;
use anyhow;

const letters: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

fn decode(input: &str) -> (i32, i32) {
    let sl = input.len();

    let mut data: VecDeque<char> = input[1..sl - 1].chars().into_iter().collect();
    let mut l = data.len();

    // dbg!(&input);
    // dbg!(&data);

    while data.len() > 0 {
        // dbg!(&data);
        // dbg!(&l);

        if data[0] == '\\' {
            if data.len() > 1 && vec!['"', '\\'].contains(&data[1]) {
                data.pop_front();
                l -= 1
            } else if data.len() > 3
                && data[1] == 'x'
                && letters.contains(data[2])
                && letters.contains(data[3])
            {
                data.pop_front();
                data.pop_front();
                data.pop_front();
                l -= 3
            }
        }
        data.pop_front();
    }

    (sl.try_into().unwrap(), l.try_into().unwrap())
}

fn encode(input: &str) -> (i32, i32) {
    let sl = input.len();

    let mut data: VecDeque<char> = input.chars().into_iter().collect();
    let mut l = data.len() + 2;

    // dbg!(&input);
    // dbg!(&data);

    while data.len() > 0 {
        // dbg!(&data);
        // dbg!(&l);

        match data[0] {
            '"' => l += 1,
            '\\' => l += 1,
            _ => {}
        }

        data.pop_front();
    }

    (l.try_into().unwrap(), sl.try_into().unwrap())
}

fn main() {
    let input = fetch::get_input(2015, 8);

    let r = input
        .lines()
        .map(|line| decode(line))
        .reduce(|a, b| ((a.0 + b.0), (a.1 + b.1)))
        .unwrap();
    println!("part 1 => {}", r.0 - r.1);

    let r = input
        .lines()
        .map(|line| encode(line))
        .reduce(|a, b| ((a.0 + b.0), (a.1 + b.1)))
        .unwrap();
    println!("part 2 => {}", r.0 - r.1)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {
        assert_eq!(decode(r#""""#), (2, 0));
        assert_eq!(decode(r#""abc""#), (5, 3));
        assert_eq!(decode(r#""aaa\"aaa""#), (10, 7));
        assert_eq!(decode(r#""\x27""#), (6, 1));
    }

    #[test]
    fn p2_tests() {
        assert_eq!(encode(r#""""#), (6, 2));
        assert_eq!(encode(r#""abc""#), (9, 5));
        assert_eq!(encode(r#""aaa\"aaa""#), (16, 10));
        assert_eq!(encode(r#""\x27""#), (11, 6));
    }
}
