use std::{num::ParseIntError, str::FromStr};

use advent::fetch;
use itertools::min;

#[derive(Clone, Copy)]
struct Box {
    length: i32,
    width: i32,
    height: i32,
}

impl FromStr for Box {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<&str> = s.split("x").collect();

        Ok(Self {
            length: dims[0].parse::<i32>()?,
            width: dims[1].parse::<i32>()?,
            height: dims[2].parse::<i32>()?,
        })
    }
}

impl Box {
    fn sides(self) -> Vec<i32> {
        vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    fn surface_area(self) -> i32 {
        self.sides().into_iter().map(|s| s * 2).sum()
    }

    fn wrapping_paper(self) -> i32 {
        let sides = self.sides();
        self.surface_area() + min(sides).unwrap()
    }

    fn perimeters(self) -> Vec<i32> {
        vec![
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ]
    }

    fn ribbon(self) -> i32 {
        let perims = self.perimeters();
        min(perims).unwrap() + self.length * self.width * self.height
    }
}

fn main() {
    let input = fetch::get_input(2015, 2);

    let total: i32 = input
        .lines()
        .map(|dim| Box::from_str(dim).unwrap())
        .map(|b| b.wrapping_paper())
        .sum();

    println!("part 1 sqft = {}", total);

    let total: i32 = input
        .lines()
        .map(|dim| Box::from_str(dim).unwrap())
        .map(|b| b.ribbon())
        .sum();

    println!("part 2 feet = {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqft_correct() {
        let b = Box {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(b.surface_area(), 52);
    }

    #[test]
    fn paper_sqft_correct() {
        let b = Box {
            length: 1,
            width: 1,
            height: 10,
        };
        assert_eq!(b.wrapping_paper(), 43);
    }

    #[test]
    fn ribbon_feet() {
        let b = Box {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(b.ribbon(), 34);
    }
}
