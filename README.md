# advent-rs

## cli installation

install [rust](https://rustup.rs/)

`cargo install --git https://github.com/kryptn/advent-rs advent-cli`

You can also clone the repo and install with `cargo install --path advent-cli`

Optionally, set the `AOC_CONFIG` environment variable to customize where inputs and your session token is stored locally. it defaults to `~/.advent_of_code/`

All inputs will be saved to `$AOC_CONFIG/input/$year/$day/input`

Cookie gets stored in `$AOC_CONFIG/.cookie`

## usage

First, get your adventofcode.com cookie. sign into adventofcode.com, inspect network requests, and look for a `session=[something]` value in a cookie header. Copy it, and run `advent set-cookie`, and paste your cookie value when prompted.

get a day's input:

`advent get 2021 1`

get a whole year's input

`advent get-year 2021`

with python, reading inputs:

```python

from pathlib import Path

def read_aoc_input(year, day):
    home = Path.home()
    with open(home / ".advent_of_code"/"input"/f"{year}"/f"{day:02}"/"input") as fd:
        return fd.read()

print(read_aoc_input(2017, 1))
```


in rust you can use the advent crate directly

`cargo add advent --git https://github.com/kryptn/advent-rs --features=fetch,parse`

```rust
use advent::input_store;

fn main() {
    let input = input_store::get_input(2017, 1);
    println!(input)
}
```


# other tools

make a new day with `./day.sh YEAR DAY` eg. `./day.sh 2015 1` or with the justfile, `just generate-day YEAR DAY`


## cleanup

format all projects: `rg -lg 'Cargo.toml' . -a | xargs -n 1 cargo fmt --manifest-path` or `just fmt-all`