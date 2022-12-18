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

fn surround_droplet(space: &Droplet) -> Droplet {
    let (lower, upper) = space.bounding_box();
    let lower = lower - (1, 1, 1).into();
    let upper = upper + (1, 1, 1).into();

    // define a space where the droplet can be inserted completely
    let mut air = Droplet::new();

    // fill the bounds with the default
    for z in lower.z..=upper.z {
        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let coord = (x, y, z).into();
                air.entry(coord).or_default();
            }
        }
    }

    // insert the lava into the empty space
    for lava_block in space.keys() {
        air.insert(*lava_block, Block::Lava);
    }

    // seed a steam block
    air.insert(lower, Block::Steam);

    // replace empty with steam until it can't find an adjacent empty block
    loop {
        let candidates: Vec<Coordinate3d> = air
            .iter()
            .filter(|(_, b)| **b == Block::Steam)
            .map(|(c, _)| c.cardinals())
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

    // replace the last empties with lava, they must be inside the droplet
    for (_, b) in air.iter_mut() {
        if *b == Block::Empty {
            *b = Block::Lava;
        }
    }

    air
}

fn delayed_print(input: &str, sep: &str, delay: std::time::Duration) {
    for part in input.split(sep) {
        print!("{part}\n");
        std::thread::sleep(delay);
    }
}

fn main() {
    let input = input_store::get_input(2022, 18);

    let points: Vec<Coordinate3d> = input.trim().lines().map(|l| parse_coordinate(l)).collect();

    let droplet: Space<Coordinate3d, Block> =
        points.into_iter().map(|p| (p, Block::Lava)).collect();

    // delayed_print(
    //     &format!("{droplet}"),
    //     "\n\n",
    //     std::time::Duration::from_millis(200),
    // );

    let part_1 = droplet
        .keys()
        .map(|k| k.cardinals())
        .flatten()
        .filter(|k| !droplet.contains_key(k))
        .count();

    println!("part_1 => {}", part_1);

    let surrounded_droplet = surround_droplet(&droplet);

    // delayed_print(
    //     &format!("{surrounded_droplet}"),
    //     "\n\n",
    //     std::time::Duration::from_millis(200),
    // );

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
