# advent-rs

## installation

`cargo install --git https://github.com/kryptn/advent-rs advent-cli`

or clone the repo and `cargo install --path advent-cli`

set the `AOC_CONFIG` environment variable to customize where inputs and your session token is stored locally. it defaults to `~/.advent_of_code/`

# other tools

make a new day with `./day.sh YEAR DAY` eg. `./day.sh 2015 1`


## cleanup

format all projects: `rg -lg 'Cargo.toml' . -a | xargs -n 1 cargo fmt --manifest-path `