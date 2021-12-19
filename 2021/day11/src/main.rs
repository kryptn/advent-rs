use std::sync::Mutex;

use advent::{
    grid::{print_grid, Coordinate, Grid},
    input_store,
};

#[derive(Debug)]
struct Octopus {
    brightness: usize,
    has_flashed: bool,
}

impl Octopus {
    fn incr(&mut self) -> bool {
        self.brightness += 1;

        if !self.has_flashed && self.brightness > 9 {
            self.has_flashed = true;
            return true;
        }

        false
    }

    fn settle(&mut self) {
        if self.has_flashed {
            self.brightness = 0;
            self.has_flashed = false;
        }
    }
}

impl From<usize> for Octopus {
    fn from(brightness: usize) -> Self {
        Self {
            brightness,
            has_flashed: false,
        }
    }
}

fn incr(cavern: &Grid<Mutex<Octopus>>, coordinate: &Coordinate) {
    if let Some(this) = cavern.get(coordinate) {
        let flashed = {
            let mut lock = this.lock().unwrap();
            lock.incr()
        };

        if flashed {
            //dbg!(coordinate);
            coordinate.neighbors().iter().for_each(|v| incr(cavern, v));
        }
    }
}

fn deconstruct(cavern: &Grid<Mutex<Octopus>>) -> Grid<usize> {
    let mut out = Grid::new();

    for (coord, value) in cavern {
        let lock = value.lock().unwrap();
        out.insert(coord.clone(), lock.brightness.clone());
    }

    out
}

fn step(cavern: &Grid<Mutex<Octopus>>) -> usize {
    cavern.keys().for_each(|k| incr(cavern, k));

    let mut flashed = 0;

    for value in cavern.values() {
        let mut lock = value.lock().unwrap();
        if lock.has_flashed {
            flashed += 1;
        }
        lock.settle();
    }

    flashed
}

fn main() {
    let input = input_store::get_input(2021, 11);

    let mut cavern: Grid<Mutex<Octopus>> = Grid::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, col) in line.trim().chars().enumerate() {
            let num: usize = col.to_string().parse().unwrap();

            cavern.insert((x as i32, y as i32).into(), Mutex::new(num.into()));
        }
    }

    // let decon = deconstruct(&cavern);
    // println!("step 0");
    // print_grid(&decon);

    let mut flashes = 0;

    let mut flashes_part1 = 0;
    let mut all_flashed = None;

    for i in 0..1_000_000 {
        let flashed = step(&cavern);
        flashes += flashed;

        if i + 1 == 100 {
            flashes_part1 = flashes;
        }
        // let decon = deconstruct(&cavern);
        // println!("step {}, flashed {} times, {} total", i + 1, flashed, flashes);
        // print_grid(&decon);

        if flashed == cavern.len() && all_flashed.is_none() {
            all_flashed = Some(i + 1);
            break;
        }
    }

    println!("part_1 => {}", flashes_part1);
    println!("part_2 => {}", all_flashed.unwrap());
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
