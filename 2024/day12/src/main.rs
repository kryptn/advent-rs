use std::collections::{HashMap, HashSet};

use advent::input_store;
use advent_toolbox::spatial::{coordinates_within, Coordinate, Space};
use colored::Colorize;
use itertools::Itertools;

const YEAR: usize = 2024;
const DAY: usize = 12;

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    //     let input = r#"
    // RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE"#;

    let fields: Space<Coordinate, char> = input.into();

    let mut part_1 = 0;

    let mut regions: HashMap<usize, HashSet<Coordinate>> = HashMap::new();
    let mut coord_to_region: HashMap<Coordinate, usize> = HashMap::new();
    let mut region_sides: HashMap<usize, usize> = HashMap::new();

    for (coord, plot) in fields.iter() {
        if coord_to_region.contains_key(&coord) {
            continue;
        }

        let edges = |space: &Space<Coordinate, char>, node: &Coordinate| -> Vec<Coordinate> {
            node.cardinals()
                .iter()
                .filter_map(|n| match space.get(n) {
                    Some(p) => {
                        if plot == p {
                            Some(n.clone())
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect()
        };

        // valid edges are those that have the same plant
        let nodes = fields.bfs(coord, &edges);
        let region_id = regions.len();
        regions.insert(region_id, nodes.clone().into_iter().collect());
        for node in nodes.iter() {
            coord_to_region.insert(node.clone(), region_id);
        }
        let area = nodes.len() as u64;
        let perimeter: u64 = nodes
            .iter()
            .map(|n| n.cardinals().iter().filter(|c| !nodes.contains(c)).count() as u64)
            .sum();
        part_1 += area * perimeter;
    }

    // convert fields to Option<char> and pad it with None
    // allows comparison of the edge of the map and outside it
    let (fields_lower, fields_upper) = fields.bounding_box();
    let one = Coordinate::new(1, 1);
    let padded_fields: Space<Coordinate, Option<char>> =
        coordinates_within(fields_lower - one, fields_upper + one)
            .iter()
            .map(|c| match fields.get(c) {
                Some(p) => (c.clone(), Some(*p)),
                None => (c.clone(), None),
            })
            .collect();

    // tuple_windows across rows and columns
    // we ultimately want to compare every two adjacent cells
    padded_fields
        .rows()
        .tuple_windows()
        .map(|(a, b)| {
            (
                a.collect::<Vec<(Coordinate, &Option<char>)>>(),
                b.collect::<Vec<(Coordinate, &Option<char>)>>(),
            )
        })
        .chain(
            padded_fields
                .columns()
                .tuple_windows()
                .map(|(a, b)| (a.collect(), b.collect())),
        )
        // we are iterating over windows of rows and columns
        .for_each(|(line_a, line_b)| {
            let mut prev_a = None;
            let mut prev_b = None;

            // zip them together to compare just two adjacent cells
            for (a, b) in line_a.into_iter().zip(line_b.into_iter()) {
                // println!("\n\n\n");
                // std::thread::sleep(std::time::Duration::from_millis(1000));

                // println!("a: {} b: {}", a.0, b.0);
                // println!("a: {:?} b: {:?}", a.1, b.1);

                // let mut a_changed = false;
                // let mut b_changed = false;

                if a.1 != b.1 {
                    // println!("  prev_a: {:?} a.1: {:?}", prev_a, a.1);
                    if a.1.is_some() && prev_b == prev_a || prev_a != *a.1 {
                        // a_changed = true;
                        let region_id = coord_to_region.get(&a.0).unwrap().clone();
                        // println!("    adding one to region {} ", region_id);
                        region_sides
                            .entry(region_id)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }

                    // println!("  prev_b: {:?} b.1: {:?}", prev_b, b.1);
                    if b.1.is_some() && prev_b == prev_a || prev_b != *b.1 {
                        // b_changed = true;
                        let region_id = coord_to_region.get(&b.0).unwrap().clone();
                        // println!("    adding one to region {} ", region_id);

                        region_sides
                            .entry(region_id)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                        // std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                }

                // let printable: Space<Coordinate, String> = padded_fields
                //     .clone()
                //     .iter()
                //     .map(|(c, p)| {
                //         let s = if p.is_some() {
                //             p.unwrap().to_string()
                //         } else {
                //             " ".to_string()
                //         };

                //         if c == &a.0 {
                //             let star = if a_changed { "*".red() } else { "*".normal() };
                //             (*c, format!("{}{s}{}", star, star))
                //         } else if c == &b.0 {
                //             let star = if b_changed { "*".green() } else { "*".normal() };
                //             (*c, format!("{}{s}{}", star, star))
                //         } else {
                //             (*c, format!(" {s} "))
                //         }
                //     })
                //     .collect();

                // println!("\n\n{}", printable);
                // std::thread::sleep(std::time::Duration::from_millis(1000));

                prev_a = *a.1;
                prev_b = *b.1;
            }
        });

    // dbg!(&regions);
    // dbg!(&region_sides);

    // for (region_id, nodes) in &regions {
    //     let sides = region_sides.get(&region_id).unwrap();
    //     let name = nodes.iter().next().unwrap();
    //     let name = fields.get(name).unwrap();

    //     println!(
    //         "A region of {name} plants with price {} * {sides} = {}",
    //         nodes.len(),
    //         nodes.len() * sides
    //     );
    // }

    let part_2: usize = region_sides
        .iter()
        .map(|(region_id, sides)| regions.get(region_id).unwrap().len() * sides)
        .sum();

    println!("part_1 => {}", part_1);
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
