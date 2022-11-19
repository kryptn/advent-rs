use advent::input_store;

enum Rule {
    Part1,
    Part2,
}

struct State {
    stack: Vec<isize>,
    cursor: usize,

    steps: usize,

    rule: Rule,
}

impl From<String> for State {
    fn from(input: String) -> Self {
        let stack = input
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect();
        let cursor = 0;
        let steps = 0;

        Self {
            stack,
            cursor,
            steps,
            rule: Rule::Part1,
        }
    }
}

impl Iterator for State {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.stack[self.cursor];
        *self.stack.get_mut(self.cursor).unwrap() += match self.rule {
            Rule::Part1 => 1,
            Rule::Part2 => {
                if offset >= 3 {
                    -1
                } else {
                    1
                }
            }
        };
        self.steps += 1;
        let cursor = self.cursor as isize + offset;

        if cursor as usize >= self.stack.len() {
            None
        } else {
            self.cursor = cursor as usize;
            Some(self.steps)
        }
    }
}

fn main() {
    let input = input_store::get_input(2017, 05);
    // let input = "0 3 0 1 -3".to_string();

    let mut state: State = input.clone().into();
    while let Some(_) = state.next() {}
    println!("part_1 => {}", state.steps);

    let mut state: State = input.into();
    state.rule = Rule::Part2;
    while let Some(_) = state.next() {}
    println!("part_2 => {}", state.steps);
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
