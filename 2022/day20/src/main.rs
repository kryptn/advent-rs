use std::{cell::RefCell, sync::Arc};

use advent::input_store;

#[derive(Debug)]
struct Number {
    value: isize,

    next: RefCell<Option<Arc<Number>>>,

    mixed_prev: RefCell<Option<Arc<Number>>>,
    mixed_next: RefCell<Option<Arc<Number>>>,
}

impl Number {
    fn new(value: isize) -> Self {
        Self {
            value,

            next: None.into(),

            mixed_prev: None.into(),
            mixed_next: None.into(),
        }
    }
}

#[derive(Debug)]
struct Numbers {
    numbers: Vec<Arc<Number>>,
    cursor: Arc<Number>,
}

impl From<Vec<isize>> for Numbers {
    fn from(input: Vec<isize>) -> Self {
        let numbers: Vec<_> = input
            .iter()
            .map(|value| Arc::new(Number::new(*value)))
            .collect();

        let get = |i: usize| -> Arc<Number> { numbers.get(i % numbers.len()).unwrap().clone() };

        let start = numbers.len();
        let end = start * 2;

        for root in start..end {
            let prev = get(root - 1);
            let next = get(root + 1);
            let root = get(root);

            root.next.replace(Some(next.clone()));
            root.mixed_prev.replace(Some(prev.clone()));
            root.mixed_next.replace(Some(next.clone()));
        }

        let cursor = numbers[0].clone();

        Self { numbers, cursor }
    }
}

impl Numbers {
    fn mix(&mut self) {
        if self.cursor.value == 0 {
            let next = self.cursor.next.borrow().clone().unwrap();
            self.cursor = next;
            return;
        }

        // set current prev to next, and next to prev
        let prev = self.cursor.mixed_prev.borrow().clone().unwrap();
        let next = self.cursor.mixed_next.borrow().clone().unwrap();
        prev.mixed_next.replace(Some(next.clone()));
        next.mixed_prev.replace(Some(prev.clone()));

        let len = self.numbers.len() as isize - 1;
        let offset = if self.cursor.value > 0 {
            self.cursor.value % len
        } else {
            let o = len + self.cursor.value;
            let m = o % len;
            (m + len) % len
        };

        let mut new_prev = self.cursor.clone();
        for _ in 0..offset {
            let next_position = new_prev.mixed_next.borrow().clone().unwrap();
            new_prev = next_position;
        }

        let new_next = new_prev
            .mixed_next
            .replace(Some(self.cursor.clone()))
            .unwrap();
        new_next.mixed_prev.replace(Some(self.cursor.clone()));

        self.cursor.mixed_next.replace(Some(new_next));
        self.cursor.mixed_prev.replace(Some(new_prev));

        let next_cursor = self.cursor.next.borrow().clone().unwrap();
        self.cursor = next_cursor;
    }

    fn round(&mut self) {
        for _ in 0..self.numbers.len() {
            self.mix()
        }
    }

    fn move_to_zero(&mut self) {
        while self.cursor.value != 0 {
            let next_cursor = self.cursor.mixed_next.borrow().clone().unwrap();
            self.cursor = next_cursor.clone();
        }
    }

    fn sum_of_three(&mut self) -> isize {
        self.move_to_zero();
        let mut out = 0;
        for i in 0..=3000 {
            let value = self.next().unwrap();
            if (i) % 1000 == 0 {
                out += value;
            }
        }
        out
    }
}

impl Iterator for Numbers {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.cursor.value;
        let next_number = self.cursor.mixed_next.borrow().clone().unwrap();
        self.cursor = next_number;
        Some(value)
    }
}

fn main() {
    let input = input_store::get_input(2022, 20);
    // let input = r#"1
    // 2
    // -3
    // 3
    // -2
    // 0
    // 4"#.to_string();
    let initial_numbers: Vec<isize> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    let mut numbers = Numbers::from(initial_numbers.clone());

    numbers.round();
    let part_1 = numbers.sum_of_three();
    println!("part_1 => {}", part_1);

    let p2_numbers: Vec<_> = initial_numbers.iter().map(|v| v * 811589153).collect();
    let mut fixed_numbers = Numbers::from(p2_numbers);
    for _ in 0..10 {
        fixed_numbers.round()
    }

    let part_2 = fixed_numbers.sum_of_three();
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
