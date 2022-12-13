use std::convert::TryFrom;

use advent::input_store;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{opt, peek},
    multi::many0,
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Node {
    Empty,
    Number(u32),
    List(Vec<Node>),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => write!(f, "_"),
            Node::List(v) => {
                let v: Vec<_> = v.iter().map(|i| format!("{}", i)).collect();
                write!(f, "[{}]", v.join(", "))
            }
            Node::Number(n) => write!(f, "{}", n),
        }
    }
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let (_, node) = parse_node(input.trim()).unwrap();
        node
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let r = match (self, other) {
            (Node::Empty, Node::Empty) => std::cmp::Ordering::Equal,
            (Node::Empty, _) => std::cmp::Ordering::Less,
            (Node::List(_) | Node::Number(_), Node::Empty) => std::cmp::Ordering::Greater,

            (Node::List(lhs), Node::Number(rhs)) => {
                Self::List(lhs.clone()).cmp(&Self::List(vec![Self::Number(*rhs)]))
            }
            (Node::Number(lhs), Node::List(rhs)) => {
                Self::List(vec![Self::Number(*lhs)]).cmp(&Self::List(rhs.clone()))
            }

            (Node::List(lhs), Node::List(rhs)) => lhs.cmp(rhs),
            (Node::Number(lhs), Node::Number(rhs)) => lhs.cmp(rhs),
        };
        r
    }
}

fn parse_node_list(input: &str) -> IResult<&str, Node> {
    // let input_clone = input.clone();
    let (input, _) = tag("[")(input)?;
    let (input, items) = many0(parse_node)(input)?;
    let (input, _) = tag("]")(input)?;

    let result = if items.len() == 0 {
        Node::List(vec![Node::Empty])
    } else {
        Node::List(items)
    };

    Ok((input, result))
}

fn parse_node_number(input: &str) -> IResult<&str, Node> {
    let (input, num) = u32(input)?;
    let result = Node::Number(num);
    Ok((input, result))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, node) = alt((parse_node_list, parse_node_number))(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, node))
}

fn main() {
    let input = input_store::get_input(2022, 13);

    // let input = r#"[1,1,3,1,1]
    // [1,1,5,1,1]

    // [[1],[2,3,4]]
    // [[1],4]

    // [9]
    // [[8,7,6]]

    // [[4,4],4,4]
    // [[4,4],4,4,4]

    // [7,7,7,7]
    // [7,7,7]

    // []
    // [3]

    // [[[]]]
    // [[]]

    // [1,[2,[3,[4,[5,6,7]]]],8,9]
    // [1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let pairs: Vec<(Node, Node)> = input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let cmps: Vec<Node> = pair.trim().lines().map(|line| Node::from(line)).collect();
            (cmps[0].clone(), cmps[1].clone())
        })
        .collect();

    let part_1: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum();

    println!("part_1 => {}", part_1);

    let input = format!("{}\n[[2]]\n[[6]]", input);
    let packets: Vec<Node> = input
        .trim()
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            match l.len() {
                0 => None,
                _ => Some(Node::from(l)),
            }
        })
        .sorted()
        .collect();

    let decoders: Vec<usize> = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == Node::from("[[2]]") || **p == Node::from("[[6]]"))
        .map(|(i, _)| i + 1)
        .collect();

    println!("part_2 => {}", decoders[0] * decoders[1]);
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
