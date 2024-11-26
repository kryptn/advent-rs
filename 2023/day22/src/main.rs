use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate3d, Point, Space};
use itertools::Itertools;

const YEAR: usize = 2023;
const DAY: usize = 22;

const DOWN: Coordinate3d = Coordinate3d { x: 0, y: 0, z: -1 };
const UP: Coordinate3d = Coordinate3d { x: 0, y: 0, z: 1 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Block(Coordinate3d, Coordinate3d);

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.0, self.1)
    }
}

fn range_intersects((a1, b1): (isize, isize), (a2, b2): (isize, isize)) -> bool {
    // println!("got ranges: ({}, {}) ({}, {})", a1, b1, a2, b2);
    let a = (a1.min(b1), a1.max(b1));
    let b = (a2.min(b2), a2.max(b2));

    let (a, b) = if a.0 < b.0 { (a, b) } else { (b, a) };
    // println!("sorted ranges: ({}, {}) ({}, {})", a.0, b.0, a.1, b.1);

    let result = a.0 <= b.0 && a.1 >= b.0;

    // println!("[{}, {}]  inx  [{}, {}]  =  {}", a1, b1, a2, b2, result);
    result
}

impl Block {
    fn intersects(&self, other: &Self) -> bool {
        let x = range_intersects((self.0.x, self.1.x), (other.0.x, other.1.x));
        let y = range_intersects((self.0.y, self.1.y), (other.0.y, other.1.y));
        let z = range_intersects((self.0.z, self.1.z), (other.0.z, other.1.z));

        x && y && z
    }

    fn above(&self) -> Self {
        let z = self.0.z.max(self.1.z) + 1;
        let a = Coordinate3d {
            x: self.0.x,
            y: self.0.y,
            z,
        };
        let b = Coordinate3d {
            x: self.1.x,
            y: self.1.y,
            z,
        };
        Self(a, b)
    }

    fn distance_from_ground(&self) -> usize {
        self.0.z.min(self.1.z) as usize
    }
}

impl Add<Coordinate3d> for Block {
    type Output = Self;

    fn add(self, rhs: Coordinate3d) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split("~");
        let start = parts.next().unwrap().parse::<Coordinate3d>().unwrap();
        let end = parts.next().unwrap().parse::<Coordinate3d>().unwrap();
        Ok(Self(start, end))
    }
}

fn breath() {
    // std::thread::sleep(std::time::Duration::from_millis(100));
}

fn settle_blocks(blocks: &HashSet<Block>) -> HashSet<Block> {
    let mut settled = HashSet::new();
    let mut unsettled: Vec<Block> = blocks.iter().cloned().collect::<Vec<_>>();

    while !unsettled.is_empty() {
        unsettled.sort_by_key(|b| b.distance_from_ground());
        let mut next_unsettled = vec![];

        // println!("unsettled: {}", unsettled.len());
        breath();
        for block in &unsettled {
            // println!("block: {}", block);
            breath();
            let mut offset = DOWN.clone();

            loop {
                // println!("offset: {}", offset);
                breath();
                let lowered = *block + offset;
                if unsettled
                    .iter()
                    .filter(|b| *b != block)
                    .any(|b: &Block| b.intersects(&lowered))
                {
                    // println!("    pushed into next");
                    breath();
                    next_unsettled.push(*block);
                    break;
                }

                if lowered.distance_from_ground() > 0
                    && settled.iter().all(|b: &Block| !b.intersects(&lowered))
                {
                    // println!("    raise offset");
                    breath();
                    offset = offset + DOWN;
                } else {
                    // println!("    pushed into settled");
                    breath();
                    settled.insert(lowered + UP);
                    break;
                }
            }
        }
        unsettled = next_unsettled;
    }

    settled
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    let input = r#"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "#;

    let mut tower: HashMap<usize, Block> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.trim().parse().unwrap()))
        .collect();

    let blocks = tower.values().cloned().collect();

    // dbg!(blocks);
    // return;
    let settled = settle_blocks(&blocks);
    let not_supporting = settled
        .iter()
        .filter(|b| {
            tower
                .values()
                .filter(|t| t != b)
                .all(|t| !t.intersects(&b.above()))
        })
        .count();

    println!("part_1 => {}", not_supporting);
    println!("part_2 => {}", "not done");
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    fn str_intersects(a: &str, b: &str) -> bool {
        let a = a.parse::<Block>().unwrap();
        let b = b.parse::<Block>().unwrap();
        a.intersects(&b)
    }

    #[rstest]
    #[case("1,1,1~0,0,0", true)]
    #[case("1,1,1~2,0,0", true)]
    #[case("1,1,1~0,2,0", true)]
    #[case("1,1,1~0,0,2", true)]
    #[case("1,1,1~2,2,0", true)]
    #[case("1,1,1~2,0,2", true)]
    #[case("1,1,1~0,2,2", true)]
    #[case("1,1,1~2,2,2", true)]
    #[case("2,2,2~2,2,2", false)]
    #[case("0,0,0~2,2,0", false)]
    fn test_block_intersects(#[case] given: &str, #[case] expected: bool) {
        let known = "1,1,1~1,1,1";
        // let block = given.parse::<Block>().unwrap();
        assert_eq!(str_intersects(known, given), expected);
    }

    #[rstest]
    #[case("1,1,1~1,1,1", "1,1,1~1,1,1")]
    #[case("1,1,2~1,1,2", "1,1,1~1,1,1")]
    #[case("1,1,3~1,1,3", "1,1,1~1,1,1")]
    #[case("1,1,4~1,1,4", "1,1,1~1,1,1")]
    #[case("1,1,5~1,1,5", "1,1,1~1,1,1")]
    #[case("1,1,6~1,1,6", "1,1,1~1,1,1")]
    fn test_settle_block(#[case] given: &str, #[case] expected: &str) {
        let blocks = given
            .trim()
            .lines()
            .map(|line| line.trim().parse::<Block>().unwrap())
            .collect::<HashSet<_>>();

        let expected = expected
            .trim()
            .lines()
            .map(|line| line.trim().parse::<Block>().unwrap())
            .collect::<HashSet<_>>();

        let settled = settle_blocks(&blocks);

        assert_eq!(settled, expected);
    }

    #[rstest]
    #[case("1,1,1~1,1,1\n1,1,2~1,1,2", "1,1,2~1,1,2")]
    #[case("1,1,1~1,1,1\n1,1,3~1,1,3", "1,1,2~1,1,2")]
    #[case("1,1,1~1,1,1\n1,1,4~1,1,4", "1,1,2~1,1,2")]
    #[case("1,1,1~1,1,1\n0,0,4~2,2,4", "0,0,2~2,2,2")]
    fn test_settle_blocks(#[case] given: &str, #[case] expected: &str) {
        let blocks = given
            .trim()
            .lines()
            .map(|line| line.trim().parse::<Block>().unwrap())
            .collect::<HashSet<_>>();

        let expected = expected
            .trim()
            .lines()
            .map(|line| line.trim().parse::<Block>().unwrap())
            .collect::<HashSet<_>>();

        let settled = settle_blocks(&blocks);

        assert!(expected.is_subset(&settled));
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
