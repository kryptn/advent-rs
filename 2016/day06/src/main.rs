use advent::input_store;
use itertools::Itertools;

fn rotate<'a>(messages: &'a str) -> Vec<String> {
    let mut out = Vec::new();

    for line in messages.lines() {
        for (idx, chr) in line.trim().chars().enumerate() {
            if out.len() == idx {
                out.push(String::from(""));
            }

            let value = out.get_mut(idx as usize).unwrap();
            value.push(chr);
        }
    }

    out
}

fn main() {
    let input = input_store::get_input(2016, 6);

    let most_common: String = rotate(&input)
        .iter()
        .map(|col| {
            let out = col
                .chars()
                .sorted()
                .dedup_by_with_count(|&a, &b| a == b)
                .sorted_by(|&a, &b| a.0.cmp(&b.0))
                .map(|(_, chr)| chr)
                //.last()  // part 1
                .nth(0) // part 2
                .unwrap();

            // dbg!(col);
            // dbg!(&out);
            out
        })
        .collect();

    println!("part 1 => {}", most_common);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_rotate() {
        let input = r#"aaaa
bbbb
cccc"#;

        let expected = vec![
            String::from("abc"),
            String::from("abc"),
            String::from("abc"),
            String::from("abc"),
        ];

        assert_eq!(rotate(input), expected);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
