use advent::{
    grid::{bounding_box, coordinates_within, Coordinate, Grid, print_grid},
    input_store,
    parsers::parse_coordinate,
};
use itertools::Itertools;

fn parse_path(input: &str) -> Vec<Coordinate> {
    input
        .trim()
        .split(" -> ")
        .map(|sp| {
            let (_, coord) = parse_coordinate(sp).unwrap();
            coord
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Object {
    Sand,
    Rock,
    Source,
    Empty,
}

impl Default for Object {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Object::Sand => "o",
            Object::Rock => "#",
            Object::Source => "+",
            Object::Empty => ".",
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SimulationState {
    FallingSand(Coordinate),
    Steady,
    Final,
}

#[derive(Debug)]
struct Cave {
    inner: Grid<Object>,
    extent: Coordinate,
    simulation: SimulationState,
}

impl From<&String> for Cave {
    fn from(input: &String) -> Self {
        let paths: Vec<Vec<Coordinate>> = input.trim().lines().map(parse_path).collect();

        let mut cave: Grid<Object> = Grid::new();
        cave.insert(SPAWN, Object::Source);

        for path in paths {
            for (a, b) in path.iter().tuple_windows() {
                for c in coordinates_within(*a, *b) {
                    cave.insert(c, Object::Rock);
                }
            }
        }

        let (_, extent) = bounding_box(&cave);
        let simulation = SimulationState::Steady;

        Self {
            inner: cave,
            extent,
            simulation,
        }
    }
}

const SPAWN: Coordinate = Coordinate { x: 500, y: 0 };

impl Cave {
    fn spawn_sand(&self) -> SimulationState {
        SimulationState::FallingSand(SPAWN)
    }

    fn is_blocked(&self, at: Coordinate) -> bool {
        match self.inner.get(&at).unwrap_or(&Object::Empty) {
            Object::Empty | Object::Source => false,
            Object::Sand | Object::Rock => true,
        }
    }

    fn fall(&self) -> SimulationState {
        let coord = match self.simulation {
            SimulationState::FallingSand(c) => c,
            _ => panic!("guaranteed"),
        };

        let fell = if self.is_blocked(coord) {
            return SimulationState::Final;
        } else if !self.is_blocked(coord.up()) {
            Some(coord.up())
        } else if !self.is_blocked(coord.up().left()) {
            Some(coord.up().left())
        } else if !self.is_blocked(coord.up().right()) {
            Some(coord.up().right())
        } else {
            None
        };

        let next_state = match fell {
            Some(new_coord) => {
                if new_coord.y >= self.extent.y {
                    SimulationState::Final
                } else {
                    SimulationState::FallingSand(new_coord)
                }
            }
            None => SimulationState::Steady,
        };
        next_state
    }

    fn next_state(&self) -> SimulationState {
        match self.simulation {
            SimulationState::FallingSand(_) => self.fall(),
            SimulationState::Steady => self.spawn_sand(),
            SimulationState::Final => SimulationState::Final,
        }
    }

    fn count_sand(&self) -> usize {
        self.inner
            .iter()
            .filter(|(_, obj)| obj == &&Object::Sand)
            .count()
    }
}

impl Iterator for Cave {
    type Item = SimulationState;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.next_state();

        match (&self.simulation, &next_state) {
            (SimulationState::FallingSand(coord), SimulationState::Steady) => {
                // went from falling sand to steady, so the sand has settled. insert new sand
                self.inner.insert(*coord, Object::Sand);
            }
            (_, SimulationState::Final) => return None,
            (SimulationState::FallingSand(_), SimulationState::FallingSand(_)) => {}
            (SimulationState::Steady, SimulationState::FallingSand(_)) => {}
            _ => panic!("probably won't happen?"),
        }

        self.simulation = next_state.clone();
        Some(next_state)
    }
}

fn main() {
    let input = input_store::get_input(2022, 14);
    // let input = r#"498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    // "#.to_string();

    let mut cave = Cave::from(&input);
    while let Some(_) = cave.next() {}
    print_grid(&cave.inner);

    println!("part_1 => {}", cave.count_sand());

    let base = cave.extent.y + 2;

    let input = format!(
        "{input}{},{} -> {},{}",
        SPAWN.x - base - 20,
        base,
        SPAWN.x + base + 20,
        base
    );

    let mut cave = Cave::from(&input);

    while let Some(_) = cave.next() {}
    print_grid(&cave.inner);
    println!("part_2 => {}", cave.count_sand());
}

#[cfg(test)]
mod test {

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
