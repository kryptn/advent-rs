use std::str::FromStr;

use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 09;


#[derive(Debug, Clone)]
struct Data(Vec<Option<u32>>);



impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Self(parse_input(value))
    }
}

impl Data {
    fn defrag(&mut self){
        self.0 = defrag(&self.0);
    }

    fn checksum(&self) -> usize {
        checksum(&self.0)
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for v in &self.0 {
            match v {
                Some(v) => out.push_str(&v.to_string()),
                None => out.push_str("."),
            }
        }
        write!(f, "{}", out)
    }
}

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut id = 0;
    let mut out = vec![];

    for (i, c) in input.trim().chars().enumerate() {
        // println!("{}: {}", i, c);
        let v = c.to_digit(10).unwrap() as u32;
        let value = match i%2 == 0 {
            true => Some(id),
            false => None,
        };

        for _ in 0..v {
            out.push(value);
        }

        if value.is_some() {
            id += 1;
        }

    }

    out
}

fn defrag(input: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut out = input.clone();

    let mut outer = input.iter().enumerate().filter(|(_, v)| v.is_some()).last().unwrap().0;

    for i in 0..out.len() {
        if i >= outer {
            break;
        }
        if out[i].is_none() {
            out.swap(i, outer);
            while out[outer].is_none() {
                outer -= 1;
            }
        }
        // println!("{}", Data(out.clone()));
    }

    out
}

fn checksum(input: &Vec<Option<u32>>) -> usize {
    input.iter().enumerate().map(|(idx, id)| {
        let o = match id {
            Some(v) => idx * *v as usize,
            None => 0,
        };
        // println!("idx: {}, id: {:?}, o: {}", idx, id, o);

        o
    }).sum()
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let data: Data = input.as_str().into();

    // let input = "2333133121414131402";
    // let data: Data = input.into();

    // println!("{}", data);

    let mut part_1 = data.clone();
    part_1.defrag();

    println!("part_1 => {}", part_1.checksum());
    println!("part_2 => {}", "not done");
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
