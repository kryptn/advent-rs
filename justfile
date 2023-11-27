
code_default := "true"
wait_default := "true"
generate-day year day code=code_default wait=wait_default:
    #!/usr/bin/env sh
    YEAR={{year}}
    DAY={{day}}
    NORMALIZED_DAY=$(printf "%02d" $DAY)
    OUT="$YEAR/day$NORMALIZED_DAY"

    mkdir -p $OUT/src
    sed 's/dayN/day'"$NORMALIZED_DAY"'/g' template/template/Cargo.toml > $OUT/Cargo.toml
    sed "s|DAY: usize = 0|DAY: usize = ${DAY}|" template/template/src/main.rs | sed "s|YEAR: usize = 0|YEAR: usize = ${YEAR}|" > $OUT/src/main.rs
    echo created $OUT

    if [ "{{code}}" = "true" ]; then
        code -a $OUT
    fi

    if [ "{{wait}}" = "true" ]; then
        advent get {{year}} {{day}} --wait
    fi

prebuild-day year day wait=wait_default:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" {{day}})
    just generate-day {{year}} {{day}} false {{wait}}
    cargo build --manifest-path {{year}}/$DAY/Cargo.toml
    cargo build --manifest-path {{year}}/$DAY/Cargo.toml --release

prebuild-year year:
    #!/usr/bin/env sh
    seq 1 25 | xargs -I{} just prebuild-day {{year}} {} false

fmt-all:
    rg -g '**/Cargo.toml' --files | xargs -I{} cargo fmt --manifest-path {}