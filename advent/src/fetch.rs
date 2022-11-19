use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{self, StatusCode};

use anyhow::{Error, Result};

use crate::input_store;
//use scraper::{Html, Selector};

fn make_client() -> Client {
    let mut headers = HeaderMap::default();

    let cookie = crate::input_store::get_cookie();
    let val = HeaderValue::from_str(cookie.trim()).unwrap();

    headers.insert("Cookie", val);

    let client = Client::builder().default_headers(headers).build().unwrap();
    client
}

fn fetch_input(selector: &input_store::Selector) -> Result<String> {
    let client = make_client();
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        selector.year, selector.day
    );
    let resp = client.get(url).send()?;

    match resp.status() {
        StatusCode::OK => Ok(resp.text()?),
        _ => Err(Error::msg("aaa")),
    }
}

pub fn submit_answer(
    selector: input_store::Selector,
    level: u16,
    answer: String,
) -> Result<String> {
    let client = make_client();
    let url = format!(
        "https://adventofcode.com/{}/day/{}/answer",
        selector.year, selector.day
    );

    let mut params = HashMap::new();
    params.insert("level", level.to_string());
    params.insert("answer", answer);

    dbg!(&params);

    let resp = client.post(url).form(&params).send()?;

    dbg!(&resp);

    Ok(resp.text()?)

    // let document = Html::parse_document(resp.text().unwrap());
    // let selector = Selector::parse("article").unwrap();
    // let selected = document.select(&selector).next().unwrap();
    // selected.inner_html()
}

pub fn get_or_fetch_input(selector: &input_store::Selector, force: bool) -> Result<String> {
    if !force && selector.exists() {
        return input_store::read_puzzle_input(&selector);
    }

    let value = fetch_input(&selector)?;
    input_store::write_puzzle_input(selector, value.clone())?;

    Ok(value)
}

pub fn get_all_inputs(year: usize, force: bool) -> Result<()> {
    for day in 1..=25 {
        let selector = input_store::Selector { year, day };
        get_or_fetch_input(&selector, force)?;
    }

    Ok(())
}

pub fn get_input(year: usize, day: usize) -> String {
    let selector = input_store::Selector { year, day };
    let puzzle_input = get_or_fetch_input(&selector, false)
        .unwrap()
        .trim()
        .to_string();
    puzzle_input
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn do_thing() {
        assert!(get_input(2020, 1).len() > 0);
    }
}
