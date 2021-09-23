use std::{collections::{HashMap, HashSet, VecDeque}, convert::TryInto, hash::{Hash, Hasher}, str::FromStr};

use advent::fetch;
use anyhow;
use itertools::Itertools;

fn look_and_say(sequence: String) -> String {
    let mut out = String::from("");

    let mut seen = 'a';
    let mut times = 0;

    for chr in sequence.trim().chars() {
        if seen == 'a' {
            times += 1;
            seen = chr;
            continue;
        }

        if seen == chr {
            times += 1;
            continue
        } else {
            out.push_str(times.to_string().as_str());
            out.push(seen);

            seen = chr;
            times = 1;
        }
    }

    out.push_str(times.to_string().as_str());
    out.push(seen);

    out
}


fn main() {
    let mut input = fetch::get_input(2015, 10);
    //let mut input = String::from("1");


    for x in 1..=50 {

        input = look_and_say(input);
        println!("step {} -> {}", x, input.len());
    }

    println!("part 1 => {}", input.len());

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
        assert_eq!(look_and_say(String::from("1")), String::from("11"));
        assert_eq!(look_and_say(String::from("11")), String::from("21"));
        assert_eq!(look_and_say(String::from("21")), String::from("1211"));
        assert_eq!(look_and_say(String::from("1211")), String::from("111221"));
        assert_eq!(look_and_say(String::from("111221")), String::from("312211"));




    }

    #[test]
    fn p2_tests() {

    }
}
