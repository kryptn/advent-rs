use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    hash::{Hash, Hasher},
    pin::Pin,
    str::FromStr,
};

use advent::fetch;
use anyhow;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(line: &str) -> Ingredient {
    let parts: Vec<&str> = line.split(" ").collect_vec();

    Ingredient {
        name: parts[0].strip_suffix(":").unwrap().to_string(),
        capacity: parts[2].strip_suffix(",").unwrap().parse::<i32>().unwrap(),
        durability: parts[4].strip_suffix(",").unwrap().parse::<i32>().unwrap(),
        flavor: parts[6].strip_suffix(",").unwrap().parse::<i32>().unwrap(),
        texture: parts[8].strip_suffix(",").unwrap().parse::<i32>().unwrap(),
        calories: parts[10].parse::<i32>().unwrap(),
    }
}

fn score(ingredients: Vec<&Ingredient>, calorie_target: i32) -> i32 {
    let mut count: HashMap<&Ingredient, i32> = HashMap::new();

    for ingredient in ingredients {
        if !count.contains_key(ingredient) {
            count.insert(ingredient, 0);
        }

        *count.get_mut(&ingredient).unwrap() += 1;
    }

    let mut super_ingredient = Ingredient {
        name: String::from("super"),
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    for (ing, val) in count {
        super_ingredient.capacity += ing.capacity * val;
        super_ingredient.durability += ing.durability * val;
        super_ingredient.flavor += ing.flavor * val;
        super_ingredient.texture += ing.texture * val;
        super_ingredient.calories += ing.calories * val;
    }

    if super_ingredient.capacity < 0 {
        super_ingredient.capacity = 0;
    }

    if super_ingredient.durability < 0 {
        super_ingredient.durability = 0;
    }

    if super_ingredient.flavor < 0 {
        super_ingredient.flavor = 0;
    }

    if super_ingredient.texture < 0 {
        super_ingredient.texture = 0;
    }

    if calorie_target > 0 && super_ingredient.calories != calorie_target {
        super_ingredient.flavor = 0;
    }

    super_ingredient.capacity
        * super_ingredient.durability
        * super_ingredient.flavor
        * super_ingredient.texture
}

fn main() {
    let input = fetch::get_input(2015, 15);
    //     let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

    let ingredients: Vec<Ingredient> = input.lines().map(|line| parse_ingredient(line)).collect();

    //let score = ingredients.iter().combinations_with_replacement(100).map(|c| score(c, 0)).max();
    let score = ingredients
        .iter()
        .combinations_with_replacement(100)
        .map(|c| score(c, 500))
        .max();
    dbg!(score);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_parse() {
        let expected = Ingredient {
            name: "Butterscotch".to_string(),
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        };
        let line = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";

        assert_eq!(parse_ingredient(line), expected);
    }

    #[test]
    fn test_distance() {
        let comet = parse_ingredient(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
        );
        //assert_eq!(comet.distance(1000), 1120);

        let comet = parse_ingredient(
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        //assert_eq!(comet.distance(1000), 1056);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
