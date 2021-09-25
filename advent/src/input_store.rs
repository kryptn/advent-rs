use std::collections::HashMap;
use std::error::Error;
use std::fs::{DirBuilder, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};

use anyhow::Result;

const AOC_CONFIG_DIR_KEY: &str = "AOC_CONFIG";

fn aoc_config_dir() -> PathBuf {
    match env::var(AOC_CONFIG_DIR_KEY) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let mut home = dirs::home_dir().unwrap();
            home.push(".advent_of_code");
            home
        }
    }
}

fn input_cache_dir() -> PathBuf {
    let mut p = aoc_config_dir();
    p.push("input");
    p
}

fn cookie_file_path() -> PathBuf {
    let mut p = aoc_config_dir();
    p.push(".cookie");
    p
}

pub struct Selector {
    pub year: u16,
    pub day: u16,
}

impl Selector {
    fn filename(&self) -> PathBuf {
        let mut input_filename = input_cache_dir();
        input_filename.push(self.year.to_string());
        input_filename.push(format!("{:02}", self.day));
        input_filename.push("input");

        input_filename
    }

    pub fn exists(&self) -> bool {
        self.filename().exists()
    }
}

pub fn get_cookie() -> String {
    let cookie_path = cookie_file_path();
    fs::read_to_string(&cookie_path).expect("error reading cookie file")
}

pub fn write_puzzle_input(selector: Selector, value: String) -> Result<()> {
    let input_filename = selector.filename();

    DirBuilder::new()
        .recursive(true)
        .create(input_filename.parent().unwrap())?;

    let mut file = File::create(input_filename)?;
    file.write_all(value.as_bytes())?;

    Ok(())
}

pub fn read_puzzle_input(selector: Selector) -> Result<String> {
    let input_filename = selector.filename();

    let file = File::open(input_filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut value = String::new();
    buf_reader.read_to_string(&mut value)?;
    Ok(value)
}

pub fn get_input(year: u16, day: u16) -> String {
    let selector = Selector { year, day };
    read_puzzle_input(selector).unwrap()
}

pub fn set_cookie(_value: String) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn do_thing() {
        assert_eq!(2, 2)
    }
}
