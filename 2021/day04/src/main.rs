use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use advent::grid;
use advent::input_store;

#[derive(Debug)]
struct Board {
    board: grid::Grid<Rc<Mutex<bool>>>,
    numbers: HashMap<u32, Rc<Mutex<bool>>>,
}

impl Board {
    fn insert(&mut self, number: u32, at: grid::Coordinate) {
        let default = Rc::new(Mutex::new(false));
        self.numbers.insert(number, default.clone());
        self.board.insert(at, default);
    }

    fn mark(&mut self, number: u32) -> Option<u32> {
        // returns score if this number triggers a win

        if let Some(block) = self.numbers.get_mut(&number) {
            if let Ok(mut lock) = block.lock() {
                *lock = true;
            }
        }

        if self.won() {
            let score = self.score();
            // dbg!(number);
            // dbg!(score);

            dbg!(number * score);
            //return Some(number * self.score());
        }

        None
    }

    fn won(&self) -> bool {
        for x in 0..5 {
            if grid::coordinates_within((x, 0).into(), (x, 4).into())
                .iter()
                .map(|c| self.board.get(c).unwrap())
                .all(|v| *v.lock().unwrap())
            {
                return true;
            }
        }

        for y in 0..5 {
            if grid::coordinates_within((0, y).into(), (4, y).into())
                .iter()
                .map(|c| self.board.get(c).unwrap())
                .all(|v| *v.lock().unwrap())
            {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let mut s = 0;
        for (num, marked) in self.numbers.iter() {
            if let Ok(ml) = marked.lock() {
                if !*ml {
                    s += num;
                }
            }
        }

        s
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let mut board = Self {
            board: grid::Grid::new(),
            numbers: HashMap::new(),
        };

        let mut y = 0;
        for line in input.lines() {
            let mut x = 0;
            for col in line.split(" ") {
                if let Ok(num) = col.trim().parse::<u32>() {
                    board.insert(num, grid::Coordinate::new(x, y));
                    x += 1;
                }
            }

            y += 1
        }

        board
    }
}

fn mark_games(boards: &mut Vec<Board>, number: u32) -> Option<u32> {
    for board in boards.iter_mut().filter(|b| !b.won()) {
        if let Some(score) = board.mark(number) {
            return Some(score);
        }
    }

    None
}

fn main() {
    let input = input_store::get_input(2021, 04);

    let mut split_input = input.trim().split("\n\n");

    let numbers: Vec<u32> = split_input
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = split_input.map(|b| b.into()).collect();

    for number in numbers {
        if let Some(score) = mark_games(&mut boards, number) {
            println!("part_1 => {}", score);
            break;
        }
    }
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
