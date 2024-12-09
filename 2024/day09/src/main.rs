use std::str::FromStr;

use advent::input_store;

const YEAR: usize = 2024;
const DAY: usize = 09;



#[derive(Debug, Clone)]
struct Segment {
    length: u32,
    data: Option<u32>,
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for _ in 0..self.length {
            match self.data {
                Some(v) => out.push_str(&v.to_string()),
                None => out.push_str("."),
            }
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone)]
struct Data(Vec<Option<u32>>);


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

fn parse_input_p1(input: &str) -> Vec<Option<u32>> {
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

fn parse_input_p2(input: &str) -> Vec<Segment> {
    let mut out = vec![];
    let mut id = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let v = c.to_digit(10).unwrap() as u32;
        let value = match i%2 == 0 {
            true => Some(id),
            false => None,
        };

        out.push(Segment {
            length: v,
            data: value,
        });

        if value.is_some() {
            id += 1;
        }



    }

    out
}

fn defrag_p1(input: &Vec<Option<u32>>) -> Vec<Option<u32>> {
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

fn defrag_p2(input: &Vec<Segment>) -> Vec<Segment> {
    let mut out = input.clone();

    let mut outer = input.iter().enumerate().filter(|(_, v)| v.data.is_some()).last().unwrap().0;
    while outer > 0 {

        // println!("{:?}", out.iter().map(|s| format!("{}", s)).collect::<Vec<String>>());
        // println!("{}\n\n", Data(convert_to_p1(&out)));

        if out[outer].data.is_none() {
            outer -= 1;
            continue;
        }
        let mut i = 0;

        while i < outer {
            // println!("i: {} -> {}, outer: {} -> {}", i, out[i], outer, out[outer]);
            if out[i].data.is_none() && out[i].length >= out[outer].length {
                out[i].data = out[outer].data;

                if out[i].length > out[outer].length {
                    out.insert(i+1, Segment {
                        length: out[i].length - out[outer].length,
                        data: None,
                    });
                    outer += 1;

                }
                out[i].length = out[outer].length;
                out[outer].data = None;

                break;
            }
            i += 1;
        }
        outer -= 1;

    }

    out
}

fn convert_to_p1(input: &Vec<Segment>) -> Vec<Option<u32>> {
    let mut out = vec![];

    for s in input {
        for _ in 0..s.length {
            out.push(s.data);
        }
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
    let data = parse_input_p1(&input);

    // let input = "2333133121414131402";
    // let data = parse_input_p1(input);

    // println!("{}", data);

    let part_1 = defrag_p1(&data);
    println!("part_1 => {}", checksum(&part_1));

    let part_2 = parse_input_p2(&input);
    let part_2 = defrag_p2(&part_2);
    let part_2 = convert_to_p1(&part_2);
    println!("part_2 => {}", checksum(&part_2));
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
