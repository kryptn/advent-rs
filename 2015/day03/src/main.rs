use std::collections::VecDeque;

use advent::{
    fetch,
    grid::{Coordinate, Grid},
};

fn deliver(k: i32, instructions: String) -> i32 {
    let mut santas = VecDeque::new();
    for _ in 0..k {
        santas.push_front(Coordinate::new(0, 0));
    }

    let mut grid = Grid::<i32>::new();
    grid.insert(Coordinate::new(0, 0), santas.len() as i32);

    for chr in instructions.chars() {
        let pos = santas[0];
        santas[0] = match chr {
            '^' => pos.up(),
            '>' => pos.right(),
            'v' => pos.down(),
            '<' => pos.left(),
            _ => unreachable!(),
        };

        if !grid.contains_key(&pos) {
            grid.insert(pos.clone(), 0);
        }
        *grid.get_mut(&pos).unwrap() += 1;

        santas.rotate_left(1);
    }

    grid.len() as i32
}

fn main() {
    let input = fetch::get_input(2015, 3);

    println!("part 1 = {} houses", deliver(1, input.clone()));
    println!("part 2 = {} houses", deliver(2, input.clone()));
}

#[cfg(test)]
mod test {
    use super::*;
}
