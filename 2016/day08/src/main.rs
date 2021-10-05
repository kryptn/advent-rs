use std::{collections::HashSet, rc::Rc};

use advent::{grid, input_store};
use itertools::Itertools;

use nom::{branch::alt, bytes::complete::tag, character::complete as ch, multi, sequence, IResult};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Instruction {
    Rect(i32, i32),
    RotateRow(i32, i32),
    RotateColumn(i32, i32),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Rect(length, width) => write!(f, "rect {}x{}", length, width),
            Instruction::RotateRow(row, by) => write!(f, "rotate row y={} by {}", row, by),
            Instruction::RotateColumn(column, by) => {
                write!(f, "rotate column x={} by {}", column, by)
            }
        }
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (_, inst) = parse_instruction(input).unwrap();
        inst
    }
}

fn parse_rect(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("rect")(input)?;
    let (input, _) = ch::space1(input)?;
    let (input, a) = ch::digit1(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, b) = ch::digit1(input)?;

    Ok((
        input,
        Instruction::Rect(a.parse().unwrap(), b.parse().unwrap()),
    ))
}

fn parse_rotate(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("rotate")(input)?;
    let (input, _) = ch::space1(input)?;
    let (input, val) = alt((tag("row"), tag("column")))(input)?;
    let (input, _) = ch::space1(input)?;
    let (input, _) = ch::one_of("xy")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, selector) = ch::digit1(input)?;
    let (input, _) = ch::space1(input)?;
    let (input, _) = tag("by")(input)?;
    let (input, _) = ch::space1(input)?;
    let (input, value) = ch::digit1(input)?;

    match val {
        "row" => Ok((
            input,
            Instruction::RotateRow(selector.parse().unwrap(), value.parse().unwrap()),
        )),
        "column" => Ok((
            input,
            Instruction::RotateColumn(selector.parse().unwrap(), value.parse().unwrap()),
        )),
        _ => unreachable!(),
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_rect, parse_rotate))(input)
}

fn rotate_col<T: Clone>(grid: &mut grid::Grid<T>, col: i32, by: i32) {
    let mut items = Vec::new();

    let (_, upper) = grid::bounding_box(grid);

    for coordinate in grid::coordinates_within(
        grid::Coordinate::new(col, 0),
        grid::Coordinate::new(col, upper.y),
    ) {
        let value = grid.get(&coordinate).unwrap().to_owned();
        items.push(value)
    }

    items.rotate_right(by as usize);

    for (&coordinate, val) in grid::coordinates_within(
        grid::Coordinate::new(col, 0),
        grid::Coordinate::new(col, upper.y),
    )
    .iter()
    .zip(items)
    {
        grid.insert(coordinate, val);
    }
}

fn rotate_row<T: Clone + std::fmt::Debug>(grid: &mut grid::Grid<T>, row: i32, by: i32) {
    let mut items = Vec::new();

    let (_, upper) = grid::bounding_box(grid);

    for coordinate in grid::coordinates_within(
        grid::Coordinate::new(0, row),
        grid::Coordinate::new(upper.x, row),
    ) {
        let value = grid.get(&coordinate).unwrap().to_owned();
        items.push(value)
    }

    items.rotate_right(by as usize);

    for (&coordinate, val) in grid::coordinates_within(
        grid::Coordinate::new(0, row),
        grid::Coordinate::new(upper.x, row),
    )
    .iter()
    .zip(items)
    {
        grid.insert(coordinate, val);
    }
}

fn rect<T: Clone>(grid: &mut grid::Grid<T>, x: i32, y: i32, val: T) {
    for coordinate in grid::coordinates_within((0, 0).into(), (x - 1, y - 1).into()) {
        grid.insert(coordinate, val.clone());
    }
}

fn apply_instruction(grid: &mut grid::Grid<bool>, instruction: Instruction) {
    match instruction {
        Instruction::Rect(x, y) => rect(grid, x, y, true),
        Instruction::RotateRow(row, by) => rotate_row(grid, row, by),
        Instruction::RotateColumn(col, by) => rotate_col(grid, col, by),
    }
}

fn make_grid(width: i32, height: i32) -> grid::Grid<bool> {
    let mut grid = grid::Grid::new();
    for coordinate in grid::coordinates_within((0, 0).into(), (width - 1, height - 1).into()) {
        grid.insert(coordinate, false);
    }
    grid
}

fn print_grid(grid: &grid::Grid<bool>) {
    let (lower, upper) = grid::bounding_box(&grid);

    // dbg!(lower);
    // dbg!(upper);

    for row in grid::iter_rows(lower, upper) {
        for cell in row {
            match grid.get(&cell) {
                Some(&val) => {
                    print!("{}", if val { "#" } else { "." })
                }
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn print_grid_with_inst(grid: &grid::Grid<bool>, inst: Instruction) {
    let (lower, upper) = grid::bounding_box(&grid);

    // dbg!(lower);
    // dbg!(upper);

    let mut highlighted_rows = Vec::new();
    let mut highlighted_cols = Vec::new();

    match inst {
        Instruction::Rect(width, height) => {
            for x in 0..width {
                highlighted_cols.push(x);
            }
            for y in 0..height {
                highlighted_rows.push(y);
            }
        }
        Instruction::RotateRow(row, by) => highlighted_rows.push(row),

        Instruction::RotateColumn(col, by) => highlighted_cols.push(col),
    }

    print!("{}\n", inst);

    print!("  ");
    for x in 0..upper.x {
        print!(
            "{}",
            if highlighted_cols.contains(&x) {
                "v"
            } else {
                " "
            }
        );
    }
    print!("\n\n");

    for row in grid::iter_rows(lower, upper) {
        let first = row.first().unwrap();
        print!(
            "{}",
            if highlighted_rows.contains(&first.y) {
                "> "
            } else {
                "  "
            }
        );
        for cell in row {
            match grid.get(&cell) {
                Some(&val) => {
                    print!("{}", if val { "#" } else { "." })
                }
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn main() {
    let input = input_store::get_input(2016, 8);
    let mut grid = make_grid(50, 6);

    //     let mut grid = make_grid(7, 3);
    //     let input = r#"rect 3x2
    // rotate column x=1 by 1
    // rotate row y=0 by 4
    // rotate column x=1 by 1"#;

    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();

    // for coordinate in grid::coordinates_within((0, 0).into(), (60, 5).into()) {
    //     grid.insert(coordinate, false);
    // }

    instructions.iter().for_each(|i| {
        apply_instruction(&mut grid, i.clone());
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        print_grid_with_inst(&grid, i.clone());
        //print_grid(&grid);

        std::thread::sleep(std::time::Duration::from_millis(50));
        println!("");
    });

    print_grid(&grid);

    let mut total = 0;
    for c in grid {
        if c.1 {
            total += 1;
        }
    }

    println!("part 1 => {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn parser() {
        let inst: Instruction = "rect 3x2".into();
        let expected = Instruction::Rect(3, 2);
        assert_eq!(expected, inst);

        let inst: Instruction = "rotate row y=0 by 3".into();
        let expected = Instruction::RotateRow(0, 3);
        assert_eq!(expected, inst);

        let inst: Instruction = "rotate column x=3 by 12".into();
        let expected = Instruction::RotateColumn(3, 12);
        assert_eq!(expected, inst);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
