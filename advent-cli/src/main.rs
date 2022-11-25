use advent::{
    fetch::{get_all_inputs, get_or_fetch_input},
    input_store::{read_puzzle_input, set_cookie, Selector},
};
use clap::{command, Parser, Subcommand};

use anyhow::Result;
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
    Get { year: usize, day: usize },

    /// Get the inputs for all of a year
    GetYear { year: usize },

    /// Show the input for a specific day
    Show { year: usize, day: usize },

    /// Set your adventofcode.com cookie
    SetCookie { cookie: Option<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { year, day } => {
            let selector = Selector { year, day };
            get_or_fetch_input(&selector, false)?;
            eprintln!("success: fetched {}-{:0>2}", year, day);
        }
        Command::GetYear { year } => {
            get_all_inputs(year, false)?;
            eprintln!("success: fetched all of year {}", year)
        }
        Command::Show { year, day } => {
            let selector = Selector { year, day };
            get_or_fetch_input(&selector, false)?;
            if let Ok(value) = read_puzzle_input(&selector) {
                println!("{}", value)
            }
        }
        Command::SetCookie { cookie } => {
            let cookie = match cookie {
                Some(value) => value,
                None => prompt_password("Your cookie value: ")?,
            };
            let cookie = if !cookie.starts_with("session=") {
                format!("session={}", cookie)
            } else {
                cookie
            };

            set_cookie(cookie, true)?;
        }
    }

    Ok(())
}
