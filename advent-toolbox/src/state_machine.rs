use std::collections::HashMap;

pub trait Machine {
    type Action;

    fn apply_mut(&mut self, action: Self::Action);
    fn next_action(&self) -> Option<Self::Action>;

    fn apply(&self, action: Self::Action) -> Self
    where
        Self: Sized + Clone,
    {
        let mut out = self.clone();
        out.apply_mut(action);
        out
    }

    fn run_mut(&mut self) {
        while let Some(action) = self.next_action() {
            self.apply_mut(action);
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inc,
    Dec,
    Jnz(usize, usize),
}

struct State {
    cursor: usize,
    instructions: Vec<Instruction>,
    memory: HashMap<usize, isize>,
}

impl Machine for State {
    type Action = Instruction;

    fn apply_mut(&mut self, action: Instruction) {
        self.cursor += 1;
        todo!()
    }

    fn next_action(&self) -> Option<Instruction> {
        if self.cursor >= self.instructions.len() {
            return None;
        }
        Some(self.instructions[self.cursor].clone())
    }
}

fn just_try() {
    let mut state = State {
        cursor: 0,
        instructions: vec![Instruction::Inc, Instruction::Dec],
        memory: HashMap::new(),
    };

    state.run_mut();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    enum TestAction {
        add(usize, isize),
        sub(usize, isize),
    }
}
