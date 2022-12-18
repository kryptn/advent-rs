use advent::input_store;
use advent_toolbox::spatial::{Coordinate3d, Space};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Block {
    Lava,
    Steam,
    Empty,
}

impl Default for Block {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Block::Lava => '#',
            Block::Steam => '.',
            Block::Empty => ' ',
        };
        write!(f, "{c}")
    }
}

type Droplet = Space<Coordinate3d, Block>;

fn parse_coordinate(line: &str) -> Coordinate3d {
    let split: Vec<_> = line.trim().split(",").collect();
    let x: isize = split[0].parse().unwrap();
    let y = split[1].parse().unwrap();
    let z = split[2].parse().unwrap();
    (x, y, z).into()
}

fn surround_droplet_v1(space: &Droplet) -> Droplet {
    let (lower, upper) = space.bounding_box();
    let lower = lower - (1, 1, 1).into();
    let upper = upper + (1, 1, 1).into();

    let mut candidates = vec![lower];
    let mut outside = Vec::new();

    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        println!("\n\n\nchecking {} neighbors", candidate);
        for neighbor in candidate.cardinals() {
            println!("  {}", neighbor);
            if space.contains_key(&neighbor) {
                println!("    in space");
                continue;
            }
            if neighbor.x < lower.x || neighbor.y < lower.y || neighbor.z < lower.z {
                println!("    out of bounds: {neighbor} < {lower}");
                continue;
            }
            if neighbor.x > upper.x || neighbor.y > upper.y || neighbor.z > upper.z {
                println!("    out of bounds: {neighbor} > {upper}");
                continue;
            }
            if outside.contains(&neighbor) {
                println!("    already evaluated");
                continue;
            }
            candidates.push(neighbor);
            println!("    added to candidates");
        }
        outside.push(candidate);
        println!("... added to outside")
    }

    let outer_steam: Droplet = outside.into_iter().map(|c| (c, Block::Steam)).collect();
    let mut filled_block = Droplet::new();

    for z in lower.z..=upper.z {
        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let coord = (x, y, z).into();
                if !outer_steam.contains_key(&coord) {
                    filled_block.insert(coord, Block::Lava);
                }
            }
        }
    }

    filled_block
}

fn surround_droplet(space: &Droplet) -> Droplet {
    let (lower, upper) = space.bounding_box();
    let lower = lower - (1, 1, 1).into();
    let upper = upper + (1, 1, 1).into();

    let mut air = Droplet::new();

    for z in lower.z..=upper.z {
        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let coord = (x, y, z).into();
                air.entry(coord).or_default();
            }
        }
    }

    for lava_block in space.keys() {
        air.insert(*lava_block, Block::Lava);
    }

    air.insert(lower, Block::Steam);

    loop {
        let candidates: Vec<Coordinate3d> = air
            .iter()
            .filter(|(c, b)| **b == Block::Steam)
            .map(|(c, b)| c.cardinals())
            .flatten()
            .filter_map(|nc| match air.get(&nc) {
                Some(Block::Empty) => Some(nc),
                _ => None,
            })
            .collect();

        if candidates.is_empty() {
            break;
        }

        for candidate in candidates {
            air.insert(candidate, Block::Steam);
        }
    }

    for (c, b) in air.iter_mut() {
        if *b == Block::Empty {
            *b = Block::Lava;
        }
    }

    // for (c, b) in air.iter_mut() {
    //     if *b == Block::Steam {
    //         *b = Block::Empty;
    //     }
    // }

    air
}

fn main() {
    let input = input_store::get_input(2022, 18);
    // let input = r#"2,2,2
    // 1,2,2
    // 3,2,2
    // 2,1,2
    // 2,3,2
    // 2,2,1
    // 2,2,3
    // 2,2,4
    // 2,2,6
    // 1,2,5
    // 3,2,5
    // 2,1,5
    // 2,3,5"#;

    let points: Vec<Coordinate3d> = input.trim().lines().map(|l| parse_coordinate(l)).collect();

    let droplet: Space<Coordinate3d, Block> =
        points.into_iter().map(|p| (p, Block::Lava)).collect();

    let part_1 = droplet
        .keys()
        .map(|k| k.cardinals())
        .flatten()
        .filter(|k| !droplet.contains_key(k))
        .count();

    println!("part_1 => {}", part_1);

    let surrounded_droplet = surround_droplet(&droplet);

    // println!("{surrounded_droplet}");

    let part_2 = droplet
        .iter()
        .filter(|(_, b)| **b == Block::Lava)
        .map(|(c, _)| c.cardinals())
        .flatten()
        .filter_map(|c| match surrounded_droplet.get(&c) {
            Some(Block::Steam) => Some(c),
            _ => None,
        })
        .count();

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
