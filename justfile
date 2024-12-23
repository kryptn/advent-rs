
code_default := "true"
wait_default := "true"
generate-day year day code=code_default wait=wait_default:
    #!/usr/bin/env sh
    YEAR={{year}}
    DAY={{day}}
    NORMALIZED_DAY=$(printf "%02d" $((10#$DAY)))
    OUT="$YEAR/day$NORMALIZED_DAY"

    mkdir -p $OUT/src
    sed 's/dayN/day'"$NORMALIZED_DAY"'/g' template/template/Cargo.toml > $OUT/Cargo.toml
    sed "s|DAY: usize = 0|DAY: usize = ${DAY}|" template/template/src/main.rs | sed "s|YEAR: usize = 0|YEAR: usize = ${YEAR}|" > $OUT/src/main.rs
    echo created $OUT

    if [ "{{code}}" = "true" ]; then
        cp advent-rs.code-workspace advent-rs.code-workspace.bak
        cat advent-rs.code-workspace  | jq --arg cargo_toml "./$OUT/Cargo.toml" 'if .settings."rust-analyzer.linkedProjects" | map(. == $cargo_toml) | any then . else .settings."rust-analyzer.linkedProjects" += [$cargo_toml] end' > advent-rs.code-workspace.new
        cp advent-rs.code-workspace.new advent-rs.code-workspace
    fi

    if [ "{{wait}}" = "true" ]; then
        advent get {{year}} {{day}} --wait
    fi

    advent show {{year}} {{day}}


prebuild-day year day wait=wait_default:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" {{day}})
    just generate-day {{year}} {{day}} false {{wait}}
    cargo build --manifest-path {{year}}/$DAY/Cargo.toml
    cargo build --manifest-path {{year}}/$DAY/Cargo.toml --release

prebuild-year year:
    #!/usr/bin/env sh
    seq 1 25 | xargs -I{} just prebuild-day {{year}} {} false

year_default := ""
fmt-all year=year_default:
    #!/usr/bin/env sh
    path="**/Cargo.toml"
    if [ year != "" ]; then
        path="{{year}}/**/Cargo.toml"
    fi

    rg -g '**/Cargo.toml' --files | xargs -I{} cargo fmt --manifest-path {}

fmt year day:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" $((10#{{day}})))
    cargo fmt --manifest-path {{year}}/$DAY/Cargo.toml

run year day *opts:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" $((10#{{day}})))
    echo "{{opts}}"
    echo "$@"
    cargo run --manifest-path {{year}}/$DAY/Cargo.toml {{opts}}

cargo year day *opts:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" $((10#{{day}})))
    cargo {{opts}} --manifest-path {{year}}/$DAY/Cargo.toml 

benchmark year day:
    #!/usr/bin/env sh
    DAY=$(printf "day%02d" $((10#{{day}})))
    cargo build --release --manifest-path {{year}}/$DAY/Cargo.toml
    hyperfine --warmup 3 -N "{{year}}/$DAY/target/release/$DAY"