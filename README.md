# advent-rs

install cli with `cargo install --features="fetch cli" --path advent`

make a new day with `./day.sh YEAR DAY` eg. `./day.sh 2015 1`



## cleanup

format all projects: `rg -lg 'Cargo.toml' . -a | xargs -n 1 cargo fmt --manifest-path `