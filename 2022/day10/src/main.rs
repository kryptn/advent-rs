use advent::{
    grid::{print_grid, Grid},
    input_store,
};

#[derive(Clone, Debug)]
struct State {
    x: i32,
    cycles: i32,
    strength: i32,
}

impl State {
    fn new() -> Self {
        let x = 1;
        let cycles = 0;
        let strength = x * cycles;

        Self {
            x,
            cycles,
            strength,
        }
    }

    fn apply(&self, inst: Instruction) -> Self {
        // println!("applying {:?} to {:?}", inst, self);
        let mut next = self.clone();
        next.cycles += 1;
        next.strength = next.x * next.cycles;
        match inst {
            Instruction::Addx(v) => next.x += v,
            _ => {}
        }
        next
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Clone, Debug)]
struct BooleanCell(bool);

impl From<bool> for BooleanCell {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl Default for BooleanCell {
    fn default() -> Self {
        Self(bool::default())
    }
}

impl std::fmt::Display for BooleanCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = if self.0 { "#" } else { "." };
        write!(f, "{}", v)
    }
}

fn main() {
    let input = input_store::get_input(2022, 10);
    // let input = r#"addx 15
    // addx -11
    // addx 6
    // addx -3
    // addx 5
    // addx -1
    // addx -8
    // addx 13
    // addx 4
    // noop
    // addx -1
    // addx 5
    // addx -1
    // addx 5
    // addx -1
    // addx 5
    // addx -1
    // addx 5
    // addx -1
    // addx -35
    // addx 1
    // addx 24
    // addx -19
    // addx 1
    // addx 16
    // addx -11
    // noop
    // noop
    // addx 21
    // addx -15
    // noop
    // noop
    // addx -3
    // addx 9
    // addx 1
    // addx -3
    // addx 8
    // addx 1
    // addx 5
    // noop
    // noop
    // noop
    // noop
    // noop
    // addx -36
    // noop
    // addx 1
    // addx 7
    // noop
    // noop
    // noop
    // addx 2
    // addx 6
    // noop
    // noop
    // noop
    // noop
    // noop
    // addx 1
    // noop
    // noop
    // addx 7
    // addx 1
    // noop
    // addx -13
    // addx 13
    // addx 7
    // noop
    // addx 1
    // addx -33
    // noop
    // noop
    // noop
    // addx 2
    // noop
    // noop
    // noop
    // addx 8
    // noop
    // addx -1
    // addx 2
    // addx 1
    // noop
    // addx 17
    // addx -9
    // addx 1
    // addx 1
    // addx -3
    // addx 11
    // noop
    // noop
    // addx 1
    // noop
    // addx 1
    // noop
    // noop
    // addx -13
    // addx -19
    // addx 1
    // addx 3
    // addx 26
    // addx -30
    // addx 12
    // addx -1
    // addx 3
    // addx 1
    // noop
    // noop
    // noop
    // addx -9
    // addx 18
    // addx 1
    // addx 2
    // noop
    // noop
    // addx 9
    // noop
    // noop
    // noop
    // addx -1
    // addx 2
    // addx -37
    // addx 1
    // addx 3
    // noop
    // addx 15
    // addx -21
    // addx 22
    // addx -6
    // addx 1
    // noop
    // addx 2
    // addx 1
    // noop
    // addx -10
    // noop
    // noop
    // addx 20
    // addx 1
    // addx 2
    // addx 2
    // addx -6
    // addx -11
    // noop
    // noop
    // noop"#;

    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|line| {
            let line: Vec<_> = line.trim().split_whitespace().collect();
            match line[0] {
                "noop" => vec![Instruction::Noop],
                "addx" => vec![
                    Instruction::Noop,
                    Instruction::Addx(line[1].parse().unwrap()),
                ],
                _ => panic!("not expecting anything else here"),
            }
        })
        .flatten()
        .collect();

    let mut state = State::new();
    let mut states = Vec::new();

    for inst in instructions {
        state = state.apply(inst);
        states.push(state.clone());
    }

    // dbg!(&states);

    let p1: i32 = (20..240)
        .step_by(40)
        .into_iter()
        .map(|idx| states[idx - 1].strength)
        .sum();
    println!("part_1 => {}", p1);

    let coords: Grid<BooleanCell> = states
        .iter()
        .enumerate()
        .map(|(idx, state)| {
            let y = (idx) / 40;
            let x = idx % 40;
            let rendered = ((x + 1) as i32 - state.x).abs() <= 1;
            // println!("cycle: {}, sprint center: {}", idx, state.x);
            ((x, y).into(), BooleanCell::from(rendered))
        })
        // .filter(|(_, r)| r.0)
        .collect();

    print_grid(&coords);

    println!("part_2 => {}", "not done");
}

#[cfg(test)]
mod test {

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
