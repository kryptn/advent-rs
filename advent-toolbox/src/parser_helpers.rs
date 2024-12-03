use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;

pub fn just_numbers<N>(input: &str) -> Vec<N>
where
    N: FromStr + Debug,
    <N as FromStr>::Err: Debug,
{
    let input: String = input
        .chars()
        .filter_map(|c| if c.is_numeric() { Some(c) } else { Some(' ') })
        .collect();

    input
        .split_whitespace()
        .map(|x| x.parse::<N>().unwrap())
        .collect()
}

pub fn lines_of_just_numbers<N>(input: &str) -> Vec<Vec<N>>
where
    N: FromStr + Debug,
    <N as FromStr>::Err: Debug,
{
    input.lines().map(just_numbers).collect()
}

pub fn columns<N>(input: &str) -> Vec<Vec<N>>
where
    N: FromStr + Debug,
    <N as FromStr>::Err: Debug,
{
    let raw: Vec<Vec<N>> = input.lines().map(just_numbers).collect();

    if raw.iter().map(|x| x.len()).unique().count() != 1 {
        panic!("Columns must be of equal length");
    }

    let mut out = vec![];
    for row in raw {
        for (i, col) in row.into_iter().enumerate() {
            if out.len() <= i {
                out.push(vec![]);
            }
            out[i].push(col);
        }
    }

    out
}
