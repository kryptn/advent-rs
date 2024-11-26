use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use advent::{input_store, parsers::ws};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::opt,
    multi::separated_list1, IResult,
};

const YEAR: usize = 2023;
const DAY: usize = 20;

// enum Module {
//     Broadcaster,
//     FlipFlop(bool),
//     Conjunction(bool),
//     Debug,
// }

// impl Module {
//     fn pulse(&mut self, input: bool) -> bool {
//         match self {
//             Module::Broadcaster => false,
//             Module::FlipFlop(state) => {
//                 if input {
//                     *state = !*state;
//                 }
//                 *state
//             }
//             Module::Conjunction(state) => {
//                 *state = input && *state;
//                 *state
//             }
//             Module::Debug => {
//                 println!("pulse: {}", input);
//                 input
//             }
//         }
//     }
// }
// struct Relay<'a> {
//     state: bool,
//     module: Module,
//     input: Vec<bool>,
//     sinks: Vec<&'a str>,
// }

// impl<'a> Relay<'a> {

//     fn send(&mut self, input: bool) {
//         self.input.push(input);
//     }

//     fn resolve(&mut self) {
//         let mut output: bool;

//         match self.module {
//             Module::Broadcaster => output = false,
//             Module::FlipFlop(_) => {
//                 println!("resolving flip-flop, current state {}, input {:?}", self.state, self.input);
//                 let input = self.input.first().unwrap();
//                 if !input { self.state = !self.state; output = self.state; }
//             },
//             Module::Conjunction(_) => {
//                 println!("resolving conjunction, current state {}, input {:?}", self.state, self.input);
//                 let input = self.input.iter().all(|i| *i);
//                 if input {

//                 }
//                 if !input { self.state = !self.state; output = self.state; }
//             },
//             Module::Debug => todo!(),
//         }

//     }
// }

// struct Machine<'a> {
//     relays: HashMap<&'a str, Relay<'a>>,
// }

// fn parse_relay(input: &str) -> IResult<&str, (&str, Relay)> {
//     let (input, module_type) = alt((tag("broadcaster"), tag("&"), tag("%")))(input)?;
//     let (input, name) = ws(alpha1)(input)?;
//     let (input, _) = ws(tag("->"))(input)?;
//     let (input, sinks) = separated_list1(tag(", "), ws(alpha1))(input)?;

//     let module = match module_type {
//         "broadcaster" => Module::Broadcaster,
//         "&" => Module::Conjunction(true),
//         "%" => Module::FlipFlop(true),
//         _ => panic!("unknown module type: {}", module_type),
//     };

//     Ok((
//         input,
//         (
//             name,
//             Relay {
//                 module,
//                 input: vec![],
//                 sinks,
//             },
//         ),
//     ))
// }

// impl Machine<'_> {
//     fn pulse(&mut self, name: &str, input: bool) {
//         let mut queue: VecDeque<&str> = vec![name].into();

//         while let Some(relay) = queue.pop_front() {
//             let relay = self.relays.get_mut(relay).unwrap();

//             relay.input.push(input);

//             if relay.input.len() == relay.sinks.len() {
//                 let output = relay
//                     .input
//                     .iter()
//                     .fold(false, |acc, input| relay.module.pulse(*input) || acc);

//                 for sink in &relay.sinks {
//                     queue.push_back(sink);
//                 }
//             }
//         }

//         let relay = self.relays.get_mut(name).unwrap();

//         let output = relay.module.pulse(input);
//         for sink in &relay.sinks {
//             self.pulse(sink, output);
//         }
//     }
// }

fn parse_neuron(input: &str) -> IResult<&str, (String, Neuron)> {
    let (input, operator_raw) = alt((tag("broadcaster"), tag("&"), tag("%")))(input)?;
    let (input, name) = opt(ws(alpha1))(input)?;
    let (input, _) = ws(tag("->"))(input)?;
    let (input, sinks) = separated_list1(tag(", "), ws(alpha1))(input)?;

    let name = name.unwrap_or("broadcaster").to_string();
    let operator = match operator_raw {
        "broadcaster" => Operator::Broadcaster,
        "&" => Operator::Conjunction,
        "%" => Operator::FlipFlop,
        _ => panic!("unknown operator type: {}", operator_raw),
    };
    let sinks = sinks.into_iter().map(|s| s.to_string()).collect();

    Ok((
        input,
        (
            name.clone(),
            Neuron {
                name,
                state: false,
                operator,
                sources: vec![],
                sinks,
            },
        ),
    ))
}

#[derive(Debug)]
enum Operator {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

struct Neuron {
    name: String,
    state: bool,
    operator: Operator,
    sources: Vec<String>,
    sinks: Vec<String>,
}

impl Neuron {
    fn oper(&self) -> String {
        match self.operator {
            Operator::Broadcaster => "broadcaster".to_string(),
            Operator::Conjunction => format!("&{}", self.name),
            Operator::FlipFlop => format!("%{}", self.name),
        }
    }
}

impl std::fmt::Display for Neuron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sinks = self.sinks.join(", ");
        write!(f, "{} -> {}", self.oper(), sinks)
    }
}

impl std::fmt::Debug for Neuron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sources = self.sources.join(", ");
        write!(f, "{}\n\tstate: {}, sources: {}", self, self.state, sources)
    }
}

struct Net(HashMap<String, Neuron>);

impl Net {
    fn pulse(
        &mut self,
        name: &str,
        input: bool,
        interrupt_on: Option<(String, bool)>,
    ) -> Option<(usize, usize)> {
        let mut pulse_queue: VecDeque<(String, bool)> = vec![(name.to_string(), input)].into();
        let mut pulses = HashMap::new();
        // *pulses.entry(false).or_insert(0) += 1;

        // println!("button -{}-> {}", if input { "high" } else { "low" }, name);

        while let Some((neuron, s)) = pulse_queue.pop_front() {
            if let Some((interrupt, state)) = &interrupt_on {
                if neuron == *interrupt && s == *state {
                    return None;
                }
            }
            *pulses.entry(s).or_insert(0) += 1;

            let neuron = match self.0.get(&neuron) {
                Some(neuron) => neuron,
                None => continue,
            };

            let sinks = neuron.sinks.clone();

            let (state, out) = match neuron.operator {
                Operator::Broadcaster => (false, Some(false)),
                Operator::Conjunction => {
                    let all_high = neuron
                        .sources
                        .iter()
                        .all(|source| self.0.get(source).unwrap().state);
                    if all_high {
                        (false, Some(false))
                    } else {
                        (true, Some(true))
                    }
                }
                Operator::FlipFlop => {
                    if !s {
                        (!neuron.state, Some(!neuron.state))
                    } else {
                        (neuron.state, None)
                    }
                }
            };

            if let Some(outgoing) = out {
                for sink in sinks {
                    // let signal = if outgoing { "high" } else { "low" };
                    // println!("{} -{}-> {}", neuron.oper(), signal, &sink);
                    pulse_queue.push_back((sink.clone(), outgoing));
                }
            }

            self.0
                .entry(neuron.name.clone())
                .and_modify(|neuron| neuron.state = state);
        }

        // println!("\n\n");

        return Some((pulses[&false], pulses[&true]));
    }
}

fn generate_net(input: String) -> Net {
    let mut neurons = input
        .lines()
        .map(|line| parse_neuron(line.trim()).unwrap().1)
        .collect::<HashMap<_, _>>();

    let connections = neurons
        .iter()
        .map(|(name, neuron)| {
            neuron
                .sinks
                .iter()
                .map(move |sink| (name.clone(), sink.clone()))
        })
        .flatten()
        .collect::<Vec<_>>();

    for (source, sink) in connections {
        neurons
            .entry(sink)
            .and_modify(|neuron| neuron.sources.push(source));
    }
    Net(neurons)
}

fn main() {
    let input = input_store::get_input(YEAR, DAY);
    // let input = r#"broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a"#;

    // let input = r#"broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output"#;

    let mut net = generate_net(input.clone());
    println!("{:#?}", net.0);

    let part_1 = (0..1000)
        .filter_map(|_| net.pulse("broadcaster", false, None))
        .fold((0, 0), |acc, (low, high)| (acc.0 + low, acc.1 + high));

    dbg!(&net.0);

    println!("low: {}, high: {}", part_1.0, part_1.1);

    println!("part_1 => {}", part_1.0 * part_1.1);

    let mut net = generate_net(input.clone());
    let mut part_2 = 1;
    while let Some(_) = net.pulse("broadcaster", false, Some(("rx".to_string(), false))) {
        part_2 += 1;
        // if i % 1000 == 0 {
        //     println!("i: {}", i);
        // }
    }

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
