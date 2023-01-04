use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use chrono::prelude::*;

use advent::{
    fetch::{get_all_inputs, get_or_fetch_input},
    input_store::{set_cookie, Selector},
};
use clap::{command, Parser, Subcommand};

use anyhow::Result;
use humantime::format_duration;
use rpassword::prompt_password;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Get a specific day's input
    Get {
        year: usize,
        day: usize,

        /// waits until the puzzle is unlocked and then runs
        #[clap(short, long, action, default_value_t = false, value_parser)]
        wait: bool,
    },

    /// Get the inputs for all of a year
    GetYear { year: usize },

    /// Show the input for a specific day
    Show {
        year: usize,
        day: usize,

        /// waits until the puzzle is unlocked and then runs
        #[clap(short, long, action, default_value_t = false, value_parser)]
        wait: bool,
    },

    /// Set your adventofcode.com cookie
    SetCookie { cookie: Option<String> },
}

fn wait_for(selector: &Selector, force: bool) -> Result<String> {
    let until = Utc
        .with_ymd_and_hms(selector.year as i32, 12, (selector.day) as u32, 5, 0, 0)
        .single()
        .unwrap()
        .naive_utc();

    loop {
        let now = Local::now();
        let now_utc = now.naive_utc();
        if now_utc < until {
            let remaining = until - now_utc;
            print!(
                "  {now} ... waiting. {}              \r",
                format_duration(remaining.to_std()?)
            );
            io::stdout().flush()?;
            sleep(Duration::from_millis(100));
        } else {
            println!("{now} ... fetching");
            break;
        }
    }

    let mut attempts = 0;
    let max_attempts = 10;

    loop {
        let result = get_or_fetch_input(&selector, force);
        if result.is_ok() || attempts >= max_attempts {
            return result;
        } else {
            println!("attempt {} failed", attempts + 1);
            attempts += 1;
            sleep(Duration::from_millis(attempts * 200));
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { year, day, wait } => {
            let selector = Selector { year, day };

            if wait {
                wait_for(&selector, false)?;
            } else {
                get_or_fetch_input(&selector, false)?;
            }

            eprintln!("success: fetched {year}-{day:0>2}");
        }
        Command::GetYear { year } => {
            get_all_inputs(year, false)?;
            eprintln!("success: fetched all of year {}", year)
        }
        Command::Show { year, day, wait } => {
            let selector = Selector { year, day };

            let value = if wait {
                wait_for(&selector, false)?
            } else {
                get_or_fetch_input(&selector, false)?
            };

            println!("{value}");
        }
        Command::SetCookie { cookie } => {
            let cookie = match cookie {
                Some(value) => value,
                None => prompt_password("Your cookie value: ")?,
            };
            let cookie = if !cookie.starts_with("session=") {
                format!("session={cookie}")
            } else {
                cookie
            };

            set_cookie(cookie, true)?;
        }
    }

    Ok(())
}
