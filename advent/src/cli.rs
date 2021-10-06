use advent::{
    fetch::{self, get_all_inputs, get_or_fetch_input},
    input_store::{read_puzzle_input, set_cookie, Selector},
};
use clap::clap_app;

use anyhow::Result;
use rpassword::prompt_password_stdout;

fn main() -> Result<()> {
    let matches = clap_app!(advent =>
        (version: "1.0")
        (author: "David Bibb <kryptn@gmail.com>")
        (about: "Fetches Advent of Code inputs")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand get =>
            (about: "get input")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@arg year: +required "Select year")
            (@arg day: "Select day")

        )
        (@subcommand cat =>
            (about: "cat input")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@arg year: "Select year")
            (@arg day: "Select day")
        )
        (@subcommand submit =>
            (about: "submit an answer")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@arg year: "Select year")
            (@arg day: "Select day")
            (@arg level: "what level")
            (@arg answer: "what level")
        )
        (@subcommand cookie =>
            (about: "set the advent cookie")
            (@arg force: -f "overwrite existing cookie")
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("get", Some(sub_m)) => {
            let year = sub_m.value_of("year").unwrap().parse::<u16>()?;

            match sub_m.value_of("day") {
                Some(day) => {
                    let day = day.parse::<u16>()?;
                    let selector = Selector { year, day };
                    let _ = get_or_fetch_input(selector)?;
                    eprintln!("success: fetched {}/{}", year, day);
                }
                None => {
                    get_all_inputs(year)?;
                    eprintln!("success: fetched all of {}", year)
                }
            }
        }
        ("cat", Some(sub_m)) => {
            let year = sub_m.value_of("year").unwrap().parse::<u16>()?;
            let day = sub_m.value_of("day").unwrap().parse::<u16>()?;

            let selector = Selector { year, day };
            match read_puzzle_input(selector) {
                Ok(value) => println!("{}", value),
                Err(e) => eprintln!(
                    "error getting input, try `advent get {} {}`\n{:?}",
                    year, day, e
                ),
            }
        }
        ("submit", Some(sub_m)) => {
            let year = sub_m.value_of("year").unwrap().parse::<u16>()?;
            let day = sub_m.value_of("day").unwrap().parse::<u16>()?;
            let level = sub_m.value_of("level").unwrap_or("1").parse::<u16>()?;

            let answer = sub_m.value_of("answer").unwrap();

            let selector = Selector { year, day };
            let v = fetch::submit_answer(selector, level, answer.to_string())?;
            dbg!(v);
        }
        ("cookie", Some(sub_m)) => {
            let cookie = prompt_password_stdout("advent of code cookie header (session=....): ")?;
            let force: bool = sub_m.is_present("force");

            set_cookie(cookie, force)?;
        }
        (_, _) => {}
    };

    // dbg!(matches);

    Ok(())
}
