use advent::{input_store, parsers::ws};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    multi::many1,
    IResult,
};

const YEAR: usize = 2023;
const DAY: usize = 6;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn winning_times(&self) -> Vec<usize> {
        let mut out = vec![];
        for speed in 0..self.time - 1 {
            let distance = speed * (self.time - speed);
            if distance > self.record {
                out.push(speed);
            }
        }
        out
    }
}

impl From<(usize, usize)> for Race {
    fn from((time, record): (usize, usize)) -> Self {
        Self { time, record }
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, numbers) = many1(ws(digit1))(input)?;
    let numbers = numbers
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    Ok((input, numbers))
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"Time:      7  15   30
    // Distance:  9  40  200"#;

    let lines: Vec<_> = input
        .trim()
        .lines()
        .map(|l| parse_line(l.trim()).unwrap().1)
        .collect();
    let races: Vec<_> = lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(&t, &r)| Race::from((t, r)))
        .collect();

    dbg!(&races);

    let winnings = races
        .iter()
        .map(|r| r.winning_times().len())
        .fold(1, |acc, x| acc * x);

    println!("part_1 => {}", winnings);

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
