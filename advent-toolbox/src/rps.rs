pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input.to_lowercase().trim() {
            "r" | "rock" | "a" | "x" => Self::Rock,
            "p" | "paper" | "b" | "y" => Self::Paper,
            "s" | "scissors" | "c" | "z" => Self::Scissors,
            _ => panic!("expected r[ock], p[aper], or s[cissors]"),
        }
    }
}

impl Hand {
    pub fn against(&self, other: &Hand) -> Condition {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Condition::Draw,
            (Hand::Rock, Hand::Paper) => Condition::Win,
            (Hand::Rock, Hand::Scissors) => Condition::Lose,
            (Hand::Paper, Hand::Rock) => Condition::Lose,
            (Hand::Paper, Hand::Paper) => Condition::Draw,
            (Hand::Paper, Hand::Scissors) => Condition::Win,
            (Hand::Scissors, Hand::Rock) => Condition::Win,
            (Hand::Scissors, Hand::Paper) => Condition::Lose,
            (Hand::Scissors, Hand::Scissors) => Condition::Draw,
        }
    }

    pub fn desired_condition(&self, cond: &Condition) -> Self {
        match (self, cond) {
            (Hand::Rock, Condition::Lose) => Self::Scissors,
            (Hand::Rock, Condition::Draw) => Self::Rock,
            (Hand::Rock, Condition::Win) => Self::Paper,
            (Hand::Paper, Condition::Lose) => Self::Rock,
            (Hand::Paper, Condition::Draw) => Self::Paper,
            (Hand::Paper, Condition::Win) => Self::Scissors,
            (Hand::Scissors, Condition::Lose) => Self::Paper,
            (Hand::Scissors, Condition::Draw) => Self::Scissors,
            (Hand::Scissors, Condition::Win) => Self::Rock,
        }
    }
}

pub enum Condition {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Condition {
    fn from(input: &str) -> Self {
        match input.to_lowercase().trim() {
            "l" | "lose" | "x" => Self::Lose,
            "d" | "draw" | "y" => Self::Draw,
            "w" | "win" | "z" => Self::Win,
            _ => panic!("expected l[ose], d[raw], or w[win]"),
        }
    }
}

#[cfg(test)]
mod test {}
