use std::collections::HashMap;

use advent::input_store;
use advent_toolbox::{
    algo::{dijkstra, dijkstra_all_paths, dijkstra_path},
    spatial::{Coordinate, Space},
};
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 21;

#[derive(Debug, Clone)]
struct Controller {
    keypad: Space<Coordinate, char>,
    rev_keypad: HashMap<char, Coordinate>,
    position: Coordinate,

    directions_to_button: HashMap<(Coordinate, char), Vec<char>>,
}

impl std::fmt::Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let fmt_space: Space<Coordinate, Button> = self
            .keypad
            .iter()
            .map(|(k, v)| {
                let active = *k == self.position;
                let button = Button { value: *v, active };
                (*k, button)
            })
            .collect();

        write!(f, "{:?}", fmt_space)
    }
}

#[derive(Debug, Clone)]
struct Button {
    value: char,
    active: bool,
}

impl std::default::Default for Button {
    fn default() -> Self {
        Self {
            value: ' ',
            active: false,
        }
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.active {
            write!(f, "[{}]", self.value)
        } else {
            write!(f, " {} ", self.value)
        }
    }
}

impl Controller {
    fn new(input: &str) -> Self {
        let mut keypad: Space<Coordinate, char> = Space::from_lines_rev(input);

        println!("{:?}", keypad);

        let a = keypad.iter().find(|(_, v)| **v == 'A').unwrap().0.clone();
        let empties = keypad
            .iter()
            .filter(|(_, v)| **v == ' ')
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        for e in empties {
            keypad.remove(&e);
        }

        let edges = |pos: &Coordinate| -> Vec<(Coordinate, Option<usize>)> {
            let cardinals = [pos.right(), pos.left(), pos.up(), pos.down()];
            cardinals
                .iter()
                .filter_map(|c| match keypad.get(c) {
                    Some(_) => Some((c.clone(), Some(1))),
                    None => None,
                })
                .collect()
        };

        let cost = |_: &Coordinate| -> Option<usize> { Some(1) };

        let mut movement_map = HashMap::new();

        for v in keypad.keys().permutations(2) {
            let start = v[0];
            let end = v[1];
            let is_goal = |c: &Coordinate| -> bool { c == end };

            let mut paths = dijkstra_all_paths(&[*start], edges, is_goal);
            // let result = dijkstra(&[*start], edges, is_goal, Some(cost));

            let turns = |a: &Vec<Coordinate>| -> usize {
                a.iter()
                    .tuple_windows()
                    .map(|(a, b)| *v[1] - *v[0])
                    .unique()
                    .count()
            };

            let compare = |a: &Vec<Coordinate>, b: &Vec<Coordinate>| turns(a).cmp(&turns(b));

            paths.sort_by(compare);

            let path = paths.first().unwrap();

            let movements = path
                .iter()
                .zip(path.iter().skip(1))
                .map(|(a, b)| {
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;
                    match (dx, dy) {
                        (0, 1) => '^',
                        (0, -1) => 'v',
                        (1, 0) => '>',
                        (-1, 0) => '<',
                        _ => panic!("unexpected movement"),
                    }
                })
                .collect::<Vec<_>>();

            movement_map.insert((*start, *keypad.get(end).unwrap()), movements);
        }

        Self {
            rev_keypad: keypad.iter().map(|(k, v)| (*v, k.clone())).collect(),
            keypad,
            position: a,
            directions_to_button: movement_map,
        }
    }

    fn press_button(&mut self, button: char) -> Vec<char> {
        // println!("button: {}", button);

        if self.rev_keypad.get(&button) == Some(&self.position) {
            return vec![];
        }

        let movements = self
            .directions_to_button
            .get(&(self.position, button))
            .unwrap()
            .clone();

        self.position = *self.rev_keypad.get(&button).unwrap();

        movements
    }
}

fn press_buttons(robots: Vec<Controller>, bottom_sequence: &str) -> String {
    let mut robots = robots;

    let mut sequence = bottom_sequence.to_string();

    for robot in robots.iter_mut() {
        println!("sequence: {}", sequence);
        let mut next_sequence = vec![];

        println!("{robot}\n\n");
        for (i, ch) in sequence.chars().enumerate() {
            let mut v = robot.press_button(ch);

            println!("sequence: {}", sequence);
            println!("          {}^", " ".repeat(i));
            v.push('A');
            println!("v: {:?}", v);
            println!("{robot}\n\n");

            next_sequence.extend(v);
        }
        sequence = next_sequence.iter().collect();
    }

    println!("sequence: {}", sequence);
    sequence
}

fn main() {
    let keypad = "789\n456\n123\n 0A";
    let arrow_keys = " ^A\n<v>";

    let keypad = Controller::new(keypad);
    let arrow_keys = Controller::new(arrow_keys);

    let mut controllers = vec![
        keypad.clone(),
        arrow_keys.clone(),
        arrow_keys.clone(),
        // arrow_keys.clone(),
    ];

    // dbg!(&keypad);

    let input = input_store::get_input(YEAR, DAY);

    let result = press_buttons(controllers, "029A");

    println!(
        "result is {} chars long, answer is {}",
        result.len(),
        result.len() * 29
    );

    println!("part_1 => {}", "not done");
    println!("part_2 => {}", "not done");
}

// example
// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//   v <<   A >>  ^ A   <   A > A  v  A   <  ^ AA > A   < v  AAA >  ^ A
//          <       A       ^   A     >        ^^   A        vvv      A
//                  0           2                   9                 A
//

//
// <v<A>A<A>^>AvA<^Av>A^A<v<A>^>AvA^A<v<A>^>AA<vA>A^A<A>A<v<A>A^>AAA<Av>A^A
//    < v <   A >  ^  > A   <   A > A   <   AA  v > A ^ A   < v  AAA ^  > A
//            <         A       ^   A       ^^      >   A        vvv      A
//                      0           2                   9                 A
//

struct Keypad {}

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
