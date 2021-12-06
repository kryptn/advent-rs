use advent::input_store;

fn fish_step(fish: Vec<u16>) -> Vec<u16> {
    let mut next: Vec<u16> = fish
        .iter()
        .map(|d| match d {
            0 => 6,
            n => n - 1,
        })
        .collect();

    for _ in 0..fish.iter().filter(|&v| *v == 0).count() {
        next.push(8);
    }

    next
}

fn main() {
    let input = input_store::get_input(2021, 06);
    //let input = "3,4,3,1,2";

    let mut fish: Vec<u16> = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    for _ in 0..80 {
        fish = fish_step(fish);
    }

    println!("part_1 => {}", fish.len());

    let mut fish: Vec<u16> = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    for _ in 0..256 {
        fish = fish_step(fish);
    }

    println!("part_2 => {}", fish.len());
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
