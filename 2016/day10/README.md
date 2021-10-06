this worked 2nd time.

```
❯ git commit -m "parser passes"
[main daa5666] parser passes
 2 files changed, 157 insertions(+)
 create mode 100644 2016/day10/Cargo.toml
 create mode 100644 2016/day10/src/main.rs
❯ cargo fmt
❯ cargo run
   Compiling day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/day10`
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:94:69
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
❯ cargo check
    Checking day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
error[E0599]: no method named `get_mut` found for fn item `for<'r> fn(&'r mut HashMap<usize, Vec<_>>, usize, _) {inner::<_>}` in the current scope
   --> src/main.rs:88:19
    |
88  |             inner.get_mut(&key).unwrap().push(value);
    |                   ^^^^^^^ method not found in `for<'r> fn(&'r mut HashMap<usize, Vec<_>>, usize, _) {inner::<_>}`
    |
   ::: /home/david/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/pin.rs:736:18
    |
736 |     pub const fn get_mut(self) -> &'a mut T
    |                  ------- the method is available for `Pin<&mut for<'r> fn(&'r mut HashMap<usize, Vec<_>>, usize, _) {inner::<_>}>` here
    |
    = note: `inner` is a function, perhaps you wish to call it
help: consider wrapping the receiver expression with the appropriate type
    |
88  |             Pin::new(&mut inner).get_mut(&key).unwrap().push(value);
    |             ^^^^^^^^^^^^^      ^

For more information about this error, try `rustc --explain E0599`.
error: could not compile `day10` due to previous error
❯ cargo fmt
❯ cargo run
   Compiling day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/day10`
thread 'main' panicked at 'index out of bounds: the len is 211 but the index is 211', src/main.rs:96:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
❯ cargo run
   Compiling day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/day10`
part 1 => 141
❯ cargo run
   Compiling day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/day10`
[src/main.rs:138] outputs = {
    13: [
        29,
    ],
    8: [
        73,
    ],
    18: [
        23,
    ],
    6: [
        17,
    ],
    14: [
        59,
    ],
    20: [
        61,
    ],
    16: [
        47,
    ],
    17: [
        71,
    ],
    19: [
        19,
    ],
    1: [
        13,
    ],
    10: [
        7,
    ],
    2: [
        31,
    ],
    0: [
        3,
    ],
    9: [
        37,
    ],
    7: [
        5,
    ],
    12: [
        43,
    ],
    15: [
        53,
    ],
    4: [
        11,
    ],
    11: [
        2,
    ],
    5: [
        67,
    ],
    3: [
        41,
    ],
}
part 1 => 141
❯ cargo run
   Compiling day10 v0.1.0 (/home/david/git/github.com/kryptn/advent-of-code-rust/2016/day10)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/day10`
part 1 => 141
part 2 => 1209
```