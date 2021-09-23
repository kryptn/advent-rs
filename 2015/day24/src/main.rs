use std::{collections::HashMap, ops::Index};

use advent::fetch;
use itertools::Itertools;

fn do_thing(vec: Vec<i64>, groups: i64) -> Option<i64> {
    let target = vec.iter().sum::<i64>() / groups;

    for i in 2..10 {
        dbg!(i);
        let valid = vec
            .iter()
            .cloned()
            .combinations(i)
            .filter(|v| {
                let s = v.iter().cloned().sum::<i64>();
                //println!("len {} target is {} sum is {}", i, target, s);
                s == target
            })
            .collect_vec();

        let v = valid
            .iter()
            .map(|v| {
                // dbg!(&v);
                match v.iter().cloned().reduce(|a, b| {
                    // dbg!(a);
                    // dbg!(b);
                    a * b
                }) {
                    Some(va) => va,
                    None => {
                        dbg!(v);
                        0
                    }
                }
            })
            .collect_vec();

        if v.len() > 0 {
            return v.iter().cloned().min();
        }

        //dbg!(v);
        // if val.len() > 1 {
        //     return val.iter().cloned().min();
        // }
    }

    None
}

fn main() {
    let input: Vec<i64> = fetch::get_input(2015, 24)
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    println!("part 1 => {:?}", do_thing(input.clone(), 3));

    println!("part 1 => {:?}", do_thing(input.clone(), 4));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
