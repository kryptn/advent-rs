use std::collections::HashSet;

use advent::input_store;
use itertools::Itertools;

use nom::{character::complete as ch, multi, sequence, IResult};

#[derive(PartialEq, Eq, Debug)]
struct AdventIp {
    outers: Vec<String>,
    inners: Vec<String>,
}

impl From<&str> for AdventIp {
    fn from(v: &str) -> Self {
        let (_, ip) = parse_line(v).unwrap();
        ip
    }
}

fn has_abba(s: &str) -> bool {
    for (a, b, bp, ap) in s.chars().tuple_windows() {
        if a != b && a == ap && b == bp {
            return true;
        }
    }
    false
}

fn get_abas(s: &str, as_bab: bool) -> Vec<String> {
    let mut abas = Vec::new();
    for (a, b, ap) in s.chars().tuple_windows() {
        if a != b && a == ap {
            if as_bab {
                let bab: String = vec![b, a, b].iter().collect();
                abas.push(bab);
            } else {
                let aba: String = vec![a, b, a].iter().collect();
                abas.push(aba);
            }
        }
    }
    abas
}

impl AdventIp {
    fn is_abba(&self) -> bool {
        self.outers.iter().any(|s| has_abba(s)) && !self.inners.iter().any(|s| has_abba(s))
    }

    fn is_aba(&self) -> bool {
        let abas: HashSet<String> = self
            .outers
            .iter()
            .map(|seg| get_abas(seg, false))
            .flatten()
            .collect();

        let babs: HashSet<String> = self
            .inners
            .iter()
            .map(|seg| get_abas(seg, true))
            .flatten()
            .collect();

        abas.intersection(&babs).collect_vec().len() > 0
    }
}

fn parse_inner(input: &str) -> IResult<&str, &str> {
    sequence::delimited(ch::char('['), ch::alpha1, ch::char(']'))(input)
}

fn parse_outer(input: &str) -> IResult<&str, &str> {
    ch::alpha1(input)
}

fn parse_line(input: &str) -> IResult<&str, AdventIp> {
    let (input, sequences) = multi::many1(sequence::tuple((parse_outer, parse_inner)))(input)?;
    let (input, last_outer) = parse_outer(input)?;

    let mut outers = Vec::new();
    let mut inners = Vec::new();

    for (outer, inner) in sequences {
        outers.push(String::from(outer));
        inners.push(String::from(inner));
    }
    outers.push(String::from(last_outer));

    Ok((input, AdventIp { outers, inners }))
}

fn main() {
    let input = input_store::get_input(2016, 7);
    let ips: Vec<AdventIp> = input.lines().map(|line| line.trim().into()).collect();

    let part_1 = ips.iter().filter(|&ip| ip.is_abba()).collect_vec().len();
    println!("part 1 => {}", part_1);

    let part_2 = ips.iter().filter(|&ip| ip.is_aba()).collect_vec().len();
    println!("part 2 => {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_parse() {
        let input = "aaa[ddd]bbb[eee]ccc";
        let expected = AdventIp {
            outers: vec!["aaa".into(), "bbb".into(), "ccc".into()],
            inners: vec!["ddd".into(), "eee".into()],
        };

        let (_, built) = parse_line(input).unwrap();

        assert_eq!(built, expected);
    }

    #[test]
    fn p1_tests() {
        let ip: AdventIp = "aaa[ddd]bbb[eee]ccc".into();
        assert_eq!(ip.is_abba(), false);
        let ip: AdventIp = "abba[mnop]qrst".into();
        assert_eq!(ip.is_abba(), true);
        let ip: AdventIp = "abcd[bddb]xyyx".into();
        assert_eq!(ip.is_abba(), false);
        let ip: AdventIp = "aaaa[qwer]tyui".into();
        assert_eq!(ip.is_abba(), false);
        let ip: AdventIp = "ioxxoj[asdfgh]zxcvbn".into();
        assert_eq!(ip.is_abba(), true);
    }

    #[test]
    fn p2_tests() {
        let ip: AdventIp = "aba[bab]xyz".into();
        assert_eq!(ip.is_aba(), true);
        let ip: AdventIp = "xyx[xyx]xyx".into();
        assert_eq!(ip.is_aba(), false);
        let ip: AdventIp = "aaa[kek]eke".into();
        assert_eq!(ip.is_aba(), true);
        let ip: AdventIp = "zazbz[bzb]cdb".into();
        assert_eq!(ip.is_aba(), true);
    }
}
