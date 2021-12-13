use advent::{
    grid::{print_grid, Axis, Coordinate, Grid},
    input_store,
    parsers::{parse_num, ws},
};
use nom::{bytes::complete::tag, character::complete::one_of, IResult};

#[derive(Debug, Clone, Default)]
struct Dot {
    value: bool,
}

impl std::fmt::Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "█" } else { " " })
    }
}

fn grid_from_input(input: &str) -> Grid<Dot> {
    let mut out = Grid::new();

    for coord in input.lines() {
        out.insert(coord.into(), Dot { value: true });
    }

    out
}

fn parse_axis(input: &str) -> IResult<&str, Axis> {
    let (input, _) = ws(tag("fold"))(input)?;
    let (input, _) = ws(tag("along"))(input)?;
    let (input, axis) = one_of("xy")(input)?;
    let (input, _) = ws(tag("="))(input)?;
    let (input, mag) = ws(parse_num)(input)?;

    Ok((input, Axis::new(axis, mag)))
}

fn folds_from_input(input: &str) -> Vec<Axis> {
    let mut folds = Vec::new();
    for fold in input.trim().lines() {
        let (_, fold) = parse_axis(fold).unwrap();
        folds.push(fold);
    }

    folds
}

fn main() {
    let input = input_store::get_input(2021, 13);

    let mut input_split = input.trim().split("\n\n");
    let dots = input_split.next().unwrap();
    let folds = input_split.next().unwrap();

    let mut dots = grid_from_input(dots);
    let folds = folds_from_input(folds);

    let mut part_1 = None;

    for fold in folds {
        let candidates: Vec<Coordinate> = dots
            .keys()
            .cloned()
            .filter(|c| match fold {
                Axis::X(v) => c.x > v,
                Axis::Y(v) => c.y > v,
            })
            .collect();

        for candidate in candidates {
            dots.remove(&candidate).unwrap();
            dots.insert(candidate.mirror(fold.clone()), Dot { value: true });
        }

        if part_1.is_none() {
            part_1 = Some(dots.len());
        }
    }

    println!("part_1 => {}", part_1.unwrap());
    println!("part_2 => read it");
    print_grid(&dots);
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
