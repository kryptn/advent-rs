use std::{fmt::Debug, str::FromStr};

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
