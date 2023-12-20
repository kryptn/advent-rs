use std::collections::HashMap;

use advent::input_store;
use advent_toolbox::parser_helpers::just_numbers;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::{many1, many_till},
    IResult,
};

const YEAR: usize = 2023;
const DAY: usize = 19;

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, target) = alpha1(input)?;
    let (input, gt_lt) = alt((tag("<"), tag(">")))(input)?;
    let (input, value) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = alpha1(input)?;
    let (input, _) = tag(",")(input)?;

    let target = target.to_string();
    let value = value.parse::<usize>().unwrap();
    let destination = destination.to_string();

    let rule = Rule {
        target,
        value,
        check: match gt_lt {
            "<" => Conditional::LessThan,
            ">" => Conditional::GreaterThan,
            _ => panic!("bad conditional"),
        },
        destination,
    };

    dbg!(&rule);

    Ok((input, rule))
}

#[derive(Debug)]
enum Conditional {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Rule {
    target: String,
    value: usize,
    check: Conditional,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    checks: Vec<Rule>,
    fallthrough: String,
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = many1(parse_rule)(input)?;
    let (input, fallthrough) = alpha1(input)?;
    let (input, _) = tag("}")(input)?;

    let workflow = Workflow {
        name: name.to_string(),
        checks: rules,
        fallthrough: fallthrough.to_string(),
    };

    Ok((input, (name.to_string(), workflow)))
}

impl Workflow {
    fn check_part(&self, part: &Part) -> String {
        for check in &self.checks {
            let target = match check.target.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => panic!("bad target"),
            };
            match check.check {
                Conditional::LessThan => {
                    if target < check.value {
                        return check.destination.clone();
                    }
                }
                Conditional::GreaterThan => {
                    if target > check.value {
                        return check.destination.clone();
                    }
                }
            }
        }

        self.fallthrough.clone()
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<String> for Part {
    fn from(value: String) -> Self {
        let numbers = just_numbers(&value);
        Self {
            x: numbers[0],
            m: numbers[1],
            a: numbers[2],
            s: numbers[3],
        }
    }
}

impl Part {
    fn evaluate(&self, workflow: &HashMap<String, Workflow>) -> bool {
        let mut step = "in".to_string();
        while workflow.contains_key(&step) {
            step = workflow[&step].check_part(self);
        }

        step == "A"
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);

    // let input = r#"px{a<2006:qkq,m>2090:A,rfg}
    // pv{a>1716:R,A}
    // lnx{m>1548:A,A}
    // rfg{s<537:gd,x>2440:R,A}
    // qs{s>3448:A,lnx}
    // qkq{x<1416:A,crn}
    // crn{x>2662:A,R}
    // in{s<1351:px,qqz}
    // qqz{s>2770:qs,m<1801:hdj,R}
    // gd{a>3333:R,R}
    // hdj{m>838:A,pv}

    // {x=787,m=2655,a=1222,s=2876}
    // {x=1679,m=44,a=2067,s=496}
    // {x=2036,m=264,a=79,s=2244}
    // {x=2461,m=1339,a=466,s=291}
    // {x=2127,m=1623,a=2188,s=1013}"#;

    let parts = input.trim().split("\n\n").collect::<Vec<&str>>();
    let workflows: HashMap<String, Workflow> = parts[0]
        .lines()
        .map(|x| parse_workflow(x.trim()).unwrap().1)
        .collect();

    let ratings: Vec<_> = parts[1]
        .lines()
        .map(|x| Part::from(x.to_string()))
        .collect();

    // dbg!(&workflows, &ratings);

    let part_1 = ratings
        .iter()
        .filter(|x| x.evaluate(&workflows))
        .map(|x| x.value())
        .sum::<usize>();

    println!("part_1 => {}", part_1);

    println!("part_2 => {}", "not done");
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
