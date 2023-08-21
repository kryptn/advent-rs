use std::collections::HashMap;

use advent::{grid, input_store};
use nom::{
    character::complete::newline,
    multi::{many0, separated_list0},
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<grid::RelativeDirection>> {
    many0(grid::parse_relative)(input)
}

fn make_keypad() -> HashMap<grid::Coordinate, i32> {
    let mut keypad = HashMap::new();

    keypad.insert(grid::Coordinate::new(-1, 1), 1);
    keypad.insert(grid::Coordinate::new(0, 1), 2);
    keypad.insert(grid::Coordinate::new(1, 1), 3);
    keypad.insert(grid::Coordinate::new(-1, 0), 4);
    keypad.insert(grid::Coordinate::new(0, 0), 5);
    keypad.insert(grid::Coordinate::new(1, 0), 6);
    keypad.insert(grid::Coordinate::new(-1, -1), 7);
    keypad.insert(grid::Coordinate::new(0, -1), 8);
    keypad.insert(grid::Coordinate::new(1, -1), 9);

    keypad
}

fn make_keypad_p2() -> HashMap<grid::Coordinate, char> {
    vec![
        ((0, 2).into(), '1'),
        ((-1, 1).into(), '2'),
        ((0, 1).into(), '3'),
        ((1, 1).into(), '4'),
        ((-2, 0).into(), '5'),
        ((-1, 0).into(), '6'),
        ((0, 0).into(), '7'),
        ((1, 0).into(), '8'),
        ((2, 0).into(), '9'),
        ((-1, -1).into(), 'A'),
        ((0, -1).into(), 'B'),
        ((1, -1).into(), 'C'),
        ((0, -2).into(), 'D'),
    ]
    .into_iter()
    .collect()
}

fn solve<'a, T>(
    keypad: &'a HashMap<grid::Coordinate, T>,
    start: grid::Coordinate,
    movements: &Vec<Vec<grid::RelativeDirection>>,
) -> Vec<&'a T> {
    let mut out = vec![];
    let mut pos = start;
    for entry in movements {
        for movement in entry {
            let next = pos + grid::Coordinate::from(*movement);
            if keypad.contains_key(&next) {
                pos = next;
            }
        }

        let val = keypad.get(&pos).expect("we shouldn't go out");
        out.push(val);
    }

    out
}

fn main() {
    let input = input_store::get_input(2016, 2);

    let movements: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            let (_, movements) = parse_line(line.trim()).unwrap();
            movements
        })
        .collect();

    //     let input = r#"ULL
    // RRDDD
    // LURDL
    // UUUUD"#;
    // let (_, movements) = parse_moves(&input).unwrap();

    let keypad = make_keypad();

    let code = solve(&keypad, (0, 0).into(), &movements);
    let code = code
        .iter()
        .map(|ch| format!("{}", ch))
        .collect::<Vec<_>>()
        .join("");
    println!("part 1 => {}", code);

    let keypad = make_keypad_p2();

    let code = solve(&keypad, (-2, 0).into(), &movements);
    let code = code
        .iter()
        .map(|ch| format!("{}", ch))
        .collect::<Vec<_>>()
        .join("");
    println!("part 2 => {}", code);
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
