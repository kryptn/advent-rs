use advent::input_store;

#[derive(Debug, Clone)]
enum NavigationStatus {
    Incomplete(String),
    Corrupted(char),
    Valid,
}

fn reduce_chunks(input: &str) -> NavigationStatus {
    let mut input = input.to_string();
    loop {
        let new = input
            .replace("()", "")
            .replace("[]", "")
            .replace("{}", "")
            .replace("<>", "");
        if new.len() == input.len() {
            break;
        }
        input = new;
    }

    let input: Vec<char> = input.chars().collect();

    for window in input.windows(2) {
        let a = window[0];
        let b = window[1];

        match (a, b) {
            ('(' | '[' | '{' | '<', ')' | ']' | '}' | '>') => {
                return NavigationStatus::Corrupted(b)
            }
            _ => {}
        }
    }

    if input.len() > 0 {
        let remaining: String = input.iter().collect();
        return NavigationStatus::Incomplete(remaining);
    }

    NavigationStatus::Valid
}

fn score_incomplete(ns: NavigationStatus) -> u64 {
    let mut s = 0;

    if let NavigationStatus::Incomplete(v) = ns {
        for chr in v.chars().rev() {
            s *= 5;
            s += match chr {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
        }
    }

    s
}

fn score(ns: &Vec<NavigationStatus>) -> u64 {
    let mut total = 0;
    for n in ns {
        match n {
            NavigationStatus::Corrupted(v) => {
                total += match v {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            }
            _ => {}
        }
    }

    total
}

fn main() {
    let input = input_store::get_input(2021, 10);

    //     let input = r#"[({(<(())[]>[[{[]{<()<>>
    // [(()[<>])]({[<{<<[]>>(
    // {([(<{}[<>[]}>{[]{[(<()>
    // (((({<>}<{<{<>}{[]{[]{}
    // [[<[([]))<([[{}[[()]]]
    // [{[{({}]{}}([{[{{{}}([]
    // {<[[]]>}<{[{[{[]{()[[[]
    // [<(<(<(<{}))><([]([]()
    // <{([([[(<>()){}]>(<<{{
    // <{([{{}}[<[[[<>{}]]]>[]]
    // "#;

    let statuses = input.trim().lines().map(|l| reduce_chunks(l)).collect();

    let this_score = score(&statuses);

    println!("part_1 => {}", this_score);

    let mut scores: Vec<u64> = statuses
        .iter()
        .cloned()
        .filter(|s| match s {
            NavigationStatus::Incomplete(_) => true,
            _ => false,
        })
        .map(|ns| score_incomplete(ns))
        .collect();
    scores.sort();

    println!("part_2 => {}", scores[scores.len() / 2]);
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
