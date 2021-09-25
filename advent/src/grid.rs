use std::{collections::HashMap, iter::Sum, ops::Add, str::FromStr};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl FromStr for CardinalDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "N" => Ok(Self::North),
            "R" | "E" => Ok(Self::East),
            "D" | "S" => Ok(Self::South),
            "L" | "W" => Ok(Self::West),
            _ => Err(anyhow::Error::msg("unknown cardinal direction")),
        }
    }
}

impl From<&str> for CardinalDirection {
    fn from(dir_str: &str) -> Self {
        Self::from_str(dir_str).unwrap()
    }
}

impl From<char> for CardinalDirection {
    fn from(dir_char: char) -> Self {
        Self::from_str(&dir_char.to_string()).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]

pub enum RelativeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for RelativeDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "N" => Ok(Self::Up),
            "R" | "E" => Ok(Self::Right),
            "D" | "S" => Ok(Self::Down),
            "L" | "W" => Ok(Self::Left),
            _ => Err(anyhow::Error::msg("unknown relative direction")),
        }
    }
}

impl From<&str> for RelativeDirection {
    fn from(dir_str: &str) -> Self {
        Self::from_str(dir_str).unwrap()
    }
}

impl From<char> for RelativeDirection {
    fn from(dir_char: char) -> Self {
        Self::from_str(&dir_char.to_string()).unwrap()
    }
}


#[cfg(feature = "parse")]
pub fn parse_cardinal(input: &str) -> nom::IResult<&str, RelativeDirection> {
    let (input, dir) = nom::character::complete::one_of("UDLR")(input)?;
    Ok((input, dir.into()))
}


impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Coordinate::new(self.x - 1, self.y - 1),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x + 1, self.y - 1),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn turn(&self, dir: RelativeDirection) -> Self {
        match dir {
            RelativeDirection::Right => Self {
                x: self.y,
                y: self.x * -1,
            },
            RelativeDirection::Left => Self {
                x: self.y * -1,
                y: self.x,
            },
            RelativeDirection::Up => self.clone(),
            RelativeDirection::Down => self.scale(-1),
        }
    }
}

impl From<CardinalDirection> for Coordinate {
    fn from(d: CardinalDirection) -> Self {
        match d {
            CardinalDirection::North => Self::new(0, 1),
            CardinalDirection::East => Self::new(1, 0),
            CardinalDirection::South => Self::new(0, -1),
            CardinalDirection::West => Self::new(-1, 0),
        }
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sum<Coordinate> for Coordinate {
    fn sum<I: Iterator<Item = Coordinate>>(iter: I) -> Self {
        iter.fold(Self::new(0, 0), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Coordinate> for Coordinate {
    fn sum<I: Iterator<Item = &'a Coordinate>>(iter: I) -> Self {
        iter.fold(Self::new(0, 0), |a, b| a + *b)
    }
}

impl Coordinate {
    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn scale(self, by: i32) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
        }
    }
}

pub fn coordinate_str(given: &str, sep: &str) -> Coordinate {
    let it: Vec<&str> = given.split(sep).collect();
    Coordinate {
        x: it[0].parse::<i32>().unwrap(),
        y: it[1].parse::<i32>().unwrap(),
    }
}

fn sorted<T: PartialEq + PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn coordinates_within(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    let mut coords: Vec<Coordinate> = Vec::new();

    let (y_left, y_right) = sorted(a.y, b.y);
    let (x_left, x_right) = sorted(a.x, b.x);

    for y in y_left..=y_right {
        for x in x_left..=x_right {
            coords.push(Coordinate::new(x, y))
        }
    }

    coords
}

pub fn manhattan(a: Coordinate, b: Coordinate) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

pub type Grid<T> = HashMap<Coordinate, T>;

pub fn new_with_size<T: PartialEq + PartialOrd + Default>(x: i32, y: i32) -> Grid<T> {
    let mut g = Grid::<T>::new();
    for coordinate in coordinates_within(Coordinate::new(0, 0), Coordinate::new(x, y)) {
        g.insert(coordinate, T::default());
    }

    g
}

pub fn bounding_box<T>(grid: &Grid<T>) -> (Coordinate, Coordinate) {
    let mut lower = Coordinate::new(0, 0);
    let mut upper = Coordinate::new(0, 0);

    for coordinate in grid.keys() {
        if coordinate.x < lower.x {
            lower.x = coordinate.x;
        }
        if coordinate.y < lower.y {
            lower.y = coordinate.y
        }

        if coordinate.x > upper.x {
            upper.x = coordinate.x;
        }
        if coordinate.y > upper.y {
            upper.y = coordinate.y
        }
    }

    (lower, upper)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range() {
        let coords = coordinates_within(Coordinate::new(0, 0), Coordinate::new(1, 1));
        let expected = vec![
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(0, 1),
            Coordinate::new(1, 1),
        ];

        assert_eq!(coords, expected);
    }

    #[test]
    fn test_fromdir() {
        let up_dir: CardinalDirection = "U".into();
        let north_coord: Coordinate = up_dir.into();

        assert_eq!(north_coord, Coordinate::new(0, 1));
    }

    #[test]
    fn test_turn_right() {
        let mut heading = Coordinate::new(0, 1);

        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(1, 0));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(0, -1));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(-1, 0));
        heading = heading.turn(RelativeDirection::Right);
        assert_eq!(heading, Coordinate::new(0, 1));
    }

    #[test]
    fn test_turn_left() {
        let mut heading = Coordinate::new(0, 1);

        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(-1, 0));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(0, -1));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(1, 0));
        heading = heading.turn(RelativeDirection::Left);
        assert_eq!(heading, Coordinate::new(0, 1));
    }

    #[test]
    fn test_with_sep() {
        let it = coordinate_str("1,2", ",");
        assert_eq!(it, Coordinate::new(1, 2));
    }
}
