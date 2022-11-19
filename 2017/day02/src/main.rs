use advent::input_store;

fn checksum(values: &Vec<u32>) -> u32 {
    let max = values.iter().max().unwrap();
    let min = values.iter().min().unwrap();
    return max - min;
}
fn mod_checksum(values: &Vec<u32>) -> u32 {
    for x in values {
        for y in values {
            if x > y && x % y == 0 {
                return x / y
            }
        }
    }
    return 0

}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let value: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    value
}

fn checksum_input(input: &str) -> u32 {
    let value = parse_input(input);
    let total = value.iter().map(|vs| checksum(vs)).sum();
    total
}

fn mod_checksum_input(input: &str) -> u32 {
    let value = parse_input(input);
    let total = value.iter().map(|vs| mod_checksum(vs)).sum();
    total
}


fn main() {
    let input = input_store::get_input(2017, 02);

    let part_1: u32 = checksum_input(&input);
    println!("part_1 => {}", part_1);

    let part_2: u32 = mod_checksum_input(&input);
    println!("part_2 => {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    const example: &str = r#"5 1 9 5
7 5 3
2 4 6 8"#;

    #[rstest]
    #[case(example, 18)]
    fn p1_tests(#[case] given: &str, #[case] expected: u32) {
        assert_eq!(checksum_input(given), expected);
    }

const example2: &str = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;

    #[rstest]
    #[case(example2, 9)]
    fn p2_tests(#[case] given: &str, #[case] expected: u32) {
        assert_eq!(mod_checksum_input(given), expected);
    }
}
