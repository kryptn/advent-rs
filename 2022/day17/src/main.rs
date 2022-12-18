use std::collections::{HashMap, VecDeque};

use advent::{
    grid::{print_grid, Coordinate, Grid, RelativeDirection},
    input_store,
};

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

struct Column {
    stopped_rocks: Grid<RockChar>,
    wind: Vec<RelativeDirection>,
    rock_cycler: Vec<Rock>,
    falling_rock: Option<FallingRock>,
    fallen_rocks: usize,
    column_width: i64,
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
        self.rock_cycler[idx]
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

        let fallen_rocks = 0;

        Self {
            stopped_rocks,
            wind,
            rock_cycler,
            falling_rock,
            fallen_rocks,
            column_width,
        }
    }

    fn check_collision(&self, shape: Rock, at: Coordinate) -> bool {
        shape.offset_coordinates(at).iter().any(|c| {
            c.x < 0 || c.x >= self.column_width || c.y < 0 || self.stopped_rocks.contains_key(&c)
        })
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

    fn gc(&mut self) {
        let mut heights: HashMap<i64, Vec<i64>> = HashMap::new();
        for coord in self.stopped_rocks.keys() {
            heights.entry(coord.x).or_default().push(coord.y);
        }

        let lowest_height = heights
            .values()
            .map(|yh| yh.iter().max().unwrap())
            .map(|y| y)
            .min()
            .unwrap();

        self.stopped_rocks = self
            .stopped_rocks
            .iter()
            .filter(|(k, v)| k.y >= lowest_height - 50)
            .map(|(&c, _)| (c, RockChar::Stopped))
            .collect();
    }
}

impl Iterator for Column {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let next_rock = match self.falling_rock {
            Some(rock) => match rock.state {
                SimulationState::Wind => {
                    let wind = self.next_wind();
                    // println!("rock is being blown {:?}", wind);

                    let next_coord = rock.coord + wind.into();

                    // dbg!(rock.coord, next_coord);

                    let next_coord = if self.check_collision(rock.kind, next_coord) {
                        // println!("rock got blocked");
                        rock.coord
                    } else {
                        // println!("rock went {:?}", wind);
                        next_coord
                    };

                    Some(FallingRock {
                        kind: rock.kind,
                        coord: next_coord,
                        state: SimulationState::Fall,
                    })
                }
                SimulationState::Fall => {
                    // println!("rock is falling");
                    let next_coord = rock.coord.down();
                    if self.check_collision(rock.kind, next_coord) {
                        // println!("rock has settled");
                        for c in rock.kind.offset_coordinates(rock.coord) {
                            self.stopped_rocks.insert(c, RockChar::Stopped);
                        }
                        self.fallen_rocks += 1;
                        None
                    } else {
                        // println!("rock fell");
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

        if self.fallen_rocks > 0 && self.fallen_rocks % 1000 == 0 {
            self.gc();
        }

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

    let part_2 = column.highest_rock() + 1;
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
