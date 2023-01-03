#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

use advent::{
    grid::{coordinates_within, print_grid, Coordinate, Grid, RelativeDirection},
    input_store,
};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Rock {
    Flat,
    Cross,
    Corner,
    Bar,
    Box,
}

impl Rock {
    fn cycle() -> Vec<Self> {
        vec![Self::Flat, Self::Cross, Self::Corner, Self::Bar, Self::Box]
    }

    fn coordinates(&self) -> Vec<Coordinate> {
        match self {
            Rock::Flat => {
                let origin = Coordinate::new(0, 0);
                let b = origin.right();
                let c = b.right();
                let d = c.right();
                vec![origin, b, c, d]
            }
            Rock::Cross => {
                let origin = Coordinate::new(0, 0);
                let center = origin.up();
                vec![origin, center, center.right(), center.up(), center.left()]
            }
            Rock::Corner => {
                let origin = Coordinate::new(0, 0);
                let left = origin.left();
                let up = origin.up();
                vec![origin, left, left.left(), up, up.up()]
            }
            Rock::Bar => {
                let origin = Coordinate::new(0, 0);
                let b = origin.up();
                let c = b.up();
                let d = c.up();
                vec![origin, b, c, d]
            }
            Rock::Box => {
                let origin = Coordinate::new(0, 0);
                vec![origin, origin.right(), origin.up(), origin.up().right()]
            }
        }
    }

    fn offset(&self) -> Coordinate {
        match self {
            Rock::Flat => (2, 0).into(),
            Rock::Cross => (3, 0).into(),
            Rock::Corner => (4, 0).into(),
            Rock::Bar => (2, 0).into(),
            Rock::Box => (2, 0).into(),
        }
    }

    fn in_bounds(&self, at: Coordinate) -> bool {
        at.y >= 0
            && match self {
                Rock::Flat => at.x >= 0 && at.x < 4,
                Rock::Cross => at.x >= 1 && at.x < 6,
                Rock::Corner => at.x >= 2 && at.x < 7,
                Rock::Bar => at.x >= 0 && at.x < 7,
                Rock::Box => at.x >= 0 && at.x < 6,
            }
    }

    fn offset_coordinates(&self, offset: Coordinate) -> Vec<Coordinate> {
        self.coordinates().iter().map(|c| *c + offset).collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct FallingRock {
    kind: Rock,
    coord: Coordinate,
    state: SimulationState,
}

impl FallingRock {}

impl From<(Rock, i64)> for FallingRock {
    fn from((rock, height): (Rock, i64)) -> Self {
        let coord = rock.offset() + (0, height).into();

        Self {
            kind: rock,
            coord,
            state: SimulationState::Wind,
        }
    }
}

#[derive(Clone, Debug)]
enum RockChar {
    Falling,
    Stopped,
    Empty,
}

impl std::fmt::Display for RockChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            RockChar::Falling => '@',
            RockChar::Stopped => '#',
            RockChar::Empty => '.',
        };
        write!(f, "{c}")
    }
}

impl Default for RockChar {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone)]
struct Column {
    stopped_rocks: Grid<RockChar>,
    wind: Vec<RelativeDirection>,
    gusts: usize,
    rock_cycler: Vec<Rock>,
    falling_rock: Option<FallingRock>,
    fallen_rocks: u128,
    column_width: i64,

    states: HashMap<ComparativeState, (i64, u128)>,
    offset: i64,
}

#[derive(Clone, Copy, Debug)]
enum SimulationState {
    Wind,
    Fall,
}

impl Column {
    fn highest_rock(&self) -> i64 {
        self.stopped_rocks.keys().map(|k| k.y).max().unwrap_or(-1)
    }

    fn next_rock(&mut self) -> Rock {
        let idx = self.fallen_rocks % 5;
        self.rock_cycler[idx as usize]
    }

    fn next_wind(&mut self) -> RelativeDirection {
        let wind = self.wind[0];
        self.wind.rotate_left(1);
        wind
    }

    fn spawn_rock(&mut self) -> Option<FallingRock> {
        let rock = self.next_rock();
        let highest = self.highest_rock();

        Some((rock, highest + 4).into())
    }

    fn new(column_width: i64, wind: Vec<RelativeDirection>) -> Self {
        let stopped_rocks = Grid::new();
        let rock_cycler = Rock::cycle();
        let falling_rock = None;

        let gusts = 0;
        let fallen_rocks = 0;

        Self {
            stopped_rocks,
            wind,
            gusts,
            rock_cycler,
            falling_rock,
            fallen_rocks,
            column_width,
            states: HashMap::new(),
            offset: 0,
        }
    }

    fn check_collision(&self, shape: Rock, at: Coordinate) -> bool {
        !shape.in_bounds(at)
            || shape
                .offset_coordinates(at)
                .iter()
                .any(|c| self.stopped_rocks.contains_key(&c))
        // shape.offset_coordinates(at).iter().any(|c| {
        //     c.x < 0 || c.x >= self.column_width || c.y < 0 || self.stopped_rocks.contains_key(&c)
        // })
    }

    fn to_full_grid(&self) -> Grid<RockChar> {
        let mut grid: Grid<RockChar> = self
            .stopped_rocks
            .keys()
            .map(|c| ((c.x, c.y * -1).into(), RockChar::Stopped))
            .collect();
        if let Some(rock) = self.falling_rock {
            for c in rock.kind.offset_coordinates(rock.coord) {
                grid.insert((c.x, c.y * -1).into(), RockChar::Falling);
            }
        }

        grid
    }

    fn check_for_cycle(&mut self) {
        if self.offset > 0 {
            return;
        }

        let state = ComparativeState::from(&self.clone());

        // println!("block_idx: {}, gust_idx: {}, blocks: {:#b}", state.block_idx, state.gust_idx, state.blocks);

        let this_height = self.highest_rock();
        let this_rock = self.fallen_rocks;

        // println!("checking for cycle at {this_rock}");

        if let Some((last_height, last_rock)) = self.states.get(&state) {
            let rock_delta = this_rock - *last_rock as u128;
            let height_delta = this_height - last_height;
            println!("found cycle at height: {this_height} ({height_delta}) rock: {this_rock} ({rock_delta})");

            let space = 1000000000000 - self.fallen_rocks;
            let times = space / rock_delta;

            self.fallen_rocks += rock_delta * times;
            self.offset = height_delta * times as i64;

            println!(
                "leaving rocks: {} and offset {}",
                self.fallen_rocks, self.offset
            );
        } else {
            self.states.insert(state, (this_height, this_rock));
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct ComparativeState {
    block_idx: usize,
    gust_idx: usize,
    blocks: u128,
}

//
//

impl From<&Column> for ComparativeState {
    fn from(column: &Column) -> Self {
        let gust_idx = column.gusts;
        let block_idx = (column.fallen_rocks % 5) as usize;
        let highest = column.highest_rock();

        let mut blocks = 0;
        for block in coordinates_within(
            (0, highest).into(),
            (column.column_width - 1, highest - 18).into(),
        ) {
            let value = if column.stopped_rocks.contains_key(&block) {
                1
            } else {
                0
            };

            blocks += value;
            blocks = blocks << 1;
        }

        Self {
            block_idx,
            gust_idx,
            blocks,
        }
    }
}

impl Iterator for Column {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let next_rock = match self.falling_rock {
            Some(rock) => match rock.state {
                SimulationState::Wind => {
                    let wind = self.wind[self.gusts];
                    // println!("rock {} is being blown {:?}", self.fallen_rocks, wind);

                    let next_coord = rock.coord + wind.into();

                    // dbg!(rock.coord, next_coord);

                    let next_coord = if self.check_collision(rock.kind, next_coord) {
                        // println!("rock {} got blocked", self.fallen_rocks);
                        rock.coord
                    } else {
                        // println!("rock {} went {:?}", self.fallen_rocks, wind);
                        next_coord
                    };

                    self.gusts = (self.gusts + 1) % self.wind.len();

                    Some(FallingRock {
                        kind: rock.kind,
                        coord: next_coord,
                        state: SimulationState::Fall,
                    })
                }
                SimulationState::Fall => {
                    // println!("rock {} is falling", self.fallen_rocks);
                    let next_coord = rock.coord.down();
                    if self.check_collision(rock.kind, next_coord) {
                        // println!("rock {} has settled", self.fallen_rocks);
                        for c in rock.kind.offset_coordinates(rock.coord) {
                            self.stopped_rocks.insert(c, RockChar::Stopped);
                        }

                        self.fallen_rocks += 1;

                        if self.fallen_rocks >= 3000 {
                            self.check_for_cycle();
                        }

                        None
                    } else {
                        // println!("rock {} fell", self.fallen_rocks);
                        Some(FallingRock {
                            kind: rock.kind,
                            coord: next_coord,
                            state: SimulationState::Wind,
                        })
                    }
                }
            },
            None => {
                // println!("no rock, spawning");
                self.spawn_rock()
            }
        };

        self.falling_rock = next_rock;

        // println!("\n\n\n\n\n");

        Some(())
    }
}

fn main() {
    let input = input_store::get_input(2022, 17);
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let gusts: Vec<RelativeDirection> = input.trim().chars().map(|c| c.into()).collect();

    let mut column = Column::new(7, gusts);

    while column.fallen_rocks < 2022 {
        column.next().unwrap();

        // let full_grid = column.to_full_grid();
        // if full_grid.len() > 0 {
        //     std::thread::sleep(std::time::Duration::from_millis(200));
        //     print_grid(&full_grid)
        // }
    }

    let part_1 = column.highest_rock() + 1;

    println!("part_1 => {}", part_1);

    while column.fallen_rocks < 1000000000000 {
        column.next().unwrap();

        // let full_grid = column.to_full_grid();
        // if full_grid.len() > 0 {
        //     print_grid(&full_grid)
        // }
    }

    let part_2 = column.highest_rock() + 1 + column.offset;
    println!("part_2 => {}", part_2);
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
