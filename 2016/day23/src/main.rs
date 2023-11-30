use advent::input_store;
use assembunny::{Register, State};

fn main() {
    let input = input_store::get_input(2016, 23);
    //     let input = r#"cpy 2 a
    // tgl a
    // tgl a
    // tgl a
    // cpy 1 a
    // dec a
    // dec a"#.to_string();

    let mut machine: State = input.into();
    machine.memory.entry(Register::A).and_modify(|v| *v = 7);
    let exhausted = machine.run();
    println!("part_1 => {}", exhausted.memory[&Register::A]);
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
