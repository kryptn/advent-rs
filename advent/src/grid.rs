use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
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
    fn test_with_sep() {
        let it = coordinate_str("1,2", ",");
        assert_eq!(it, Coordinate::new(1, 2));
    }
}
