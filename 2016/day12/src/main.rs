use advent::input_store;
use assembunny::{Register, State};

fn main() {
    let input = input_store::get_input(2016, 12);
    // let input = r#"cpy 41 a
    // inc a
    // inc a
    // dec a
    // jnz a 2
    // dec a"#;
    //let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let first_state: State = input.into();

    let mut state: Option<State> = Some(first_state.clone());
    loop {
        let next_state = state.clone().unwrap().step();
        //dbg!(state.clone().unwrap().memory.clone());
        state = match next_state {
            Some(s) => Some(s),
            None => break,
        }
    }
    println!("part 1 => {}", state.unwrap().memory[&Register::A]);

    let mut state = Some(State {
        memory: first_state.memory_with(Register::C, 1),
        ..first_state
    });
    loop {
        let next_state = state.clone().unwrap().step();
        //dbg!(state.clone().unwrap().memory.clone());
        state = match next_state {
            Some(s) => Some(s),
            None => break,
        }
    }

    println!("part 2 => {}", state.unwrap().memory[&Register::A]);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

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
