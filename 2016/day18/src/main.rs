use advent::input_store;
use itertools::{self, Itertools};

fn next_row(row: &str) -> String {
    let mut out = String::new();
    let row = format!(".{}.", row);

    for (l, c, r) in row.chars().tuple_windows() {
        if l == c && c != r || r == c && c != l {
            out.push_str("^");
        } else {
            out.push_str(".");
        }
    }
    out
}

struct RowIterator {
    state: Option<String>,
    starting_row: String,
}

impl From<String> for RowIterator {
    fn from(row: String) -> Self {
        Self {
            state: None,
            starting_row: row,
        }
    }
}

impl Iterator for RowIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.state {
            Some(state) => self.state = Some(next_row(&state)),
            None => self.state = Some(self.starting_row.clone()),
        };
        self.state.clone()
    }
}

fn count_safes(input: String, k: u32) -> u32 {
    let rowiter: RowIterator = input.into();

    let mut safes = 0;

    for row in rowiter.take(k as usize) {
        for c in row.chars() {
            if c == '.' {
                safes += 1;
            }
        }
    }

    safes
}

fn main() {
    let input = input_store::get_input(2016, 18).trim().to_string();

    let total = count_safes(input.clone(), 40);
    println!("part 1 => {}", total);

    let total = count_safes(input, 400000);
    println!("part 2 => {}", total);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[rstest]
    #[case("..^^.", ".^^^^")]
    #[case(".^^^^", "^^..^")]
    fn next_row_test(#[case] row: &str, #[case] expected: String) {
        assert_eq!(next_row(row), expected);
    }

    #[test]
    fn test_counter() {
        let input = String::from(".^^.^.^^^^");
        assert_eq!(count_safes(input, 10), 38);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p1_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
