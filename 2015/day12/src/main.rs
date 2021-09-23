use std::{any, collections::{HashMap, HashSet, VecDeque}, convert::TryInto, hash::{Hash, Hasher}, ops::Index, str::FromStr};

use advent::fetch;
use anyhow;
use serde_json::{Map, Result, Value, json, map::Values};

fn deep_sum(value: Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(items) => items.iter().map(|v| deep_sum(v.to_owned())).sum(),
        Value::Object(items) => items.values().map(|v| deep_sum(v.to_owned())).sum(),
        _ => 0,
    }
}

fn deep_sum_without_red(value: Value) -> i64 {

    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(items) => items.iter().map(|v| deep_sum_without_red(v.to_owned())).sum(),
        Value::Object(items) => {
            if items.iter().any(|(_k, v)| *v == Value::String(String::from("red"))) {
                0
            } else {
                items.iter().map(|(_k, v)| deep_sum_without_red(v.to_owned())).sum()
            }
        },
        _ => 0,
    }
}

fn main() {
    let input = fetch::get_input(2015, 12);
    //let input = String::from(r#"[1,{"c":"red","b":2},3]"#);
    let value = serde_json::from_str(input.as_str()).unwrap();
    println!("part 1 -> {}", deep_sum(value));

    let value = serde_json::from_str(input.as_str()).unwrap();
    println!("part 2 -> {}", deep_sum_without_red(value));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
