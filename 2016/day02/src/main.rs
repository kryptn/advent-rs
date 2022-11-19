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

fn parse_moves(input: &str) -> IResult<&str, Vec<Vec<grid::RelativeDirection>>> {
    separated_list0(newline, parse_line)(input)
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

fn main() {
    let input = input_store::get_input(2016, 2);
    //     let input = r#"ULL
    // RRDDD
    // LURDL
    // UUUUD"#;
    let (_, movements) = parse_moves(&input).unwrap();

    let keypad = make_keypad();

    let mut code = String::new();
    let mut pos = grid::Coordinate::new(0, 0);
    for entry in movements {
        // dbg!(&entry);

        println!("\n\nstarting row at {:?}", pos);
        for movement in entry {
            let next = pos + movement.into();
            //dbg!(next);
            if keypad.contains_key(&next) {
                pos = next;
                println!("went {:?}, at {:?}", movement, pos);
            } else {
                println!("could not move {:?}, still at {:?}", movement, pos);
            }
        }

        println!("ending row at {:?}", pos);

        let val = keypad.get(&pos).expect("we shouldn't go out").to_owned();
        code.push_str(&val.to_string())
    }

    println!("part 1 => {}", code);
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
