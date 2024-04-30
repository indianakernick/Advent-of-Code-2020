use std::collections::{HashMap, VecDeque};

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut modules = HashMap::new();
    let mut broadcaster_destinations = Vec::new();

    for line in common::lines_iter(input) {
        if line[0] == b'%' {
            let arrow = common::index_of(line, b'-');
            modules.insert(&line[1..arrow - 1], Module {
                destinations: parse_destinations(&line[arrow + 3..]),
                brain: ModuleBrain::FlipFlop(FlipFlopModule { state: false }),
            });
        } else if line[0] == b'&' {
            let arrow = common::index_of(line, b'-');
            modules.insert(&line[1..arrow - 1], Module {
                destinations: parse_destinations(&line[arrow + 3..]),
                brain: ModuleBrain::Conjunction(ConjunctionModule { inputs: HashMap::new() }),
            });
        } else if line.starts_with(b"broadcaster") {
            broadcaster_destinations = parse_destinations(&line[15..]);
        } else {
            panic!("Invalid input");
        }
    }

    let mut queue = VecDeque::new();

    for (id, module) in modules.iter() {
        for dest in module.destinations.iter() {
            queue.push_back((*id, Pulse::Low, *dest));
        }
    }

    for (from, _, to) in queue.drain(..) {
        if let Some(to_module) = modules.get_mut(to) {
            if let ModuleBrain::Conjunction(b) = &mut to_module.brain {
                b.init_input(from);
            }
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        low_count += 1; // Button to broadcaster.

        for dest in broadcaster_destinations.iter() {
            queue.push_back((b"broadcaster".as_slice(), Pulse::Low, *dest));
        }

        while let Some((from, pulse, to)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            if let Some(to_module) = modules.get_mut(to) {
                let output = match &mut to_module.brain {
                    ModuleBrain::FlipFlop(b) => b.process(pulse),
                    ModuleBrain::Conjunction(b) => Some(b.process(from, pulse)),
                };

                if let Some(output) = output {
                    for next_to in to_module.destinations.iter() {
                        queue.push_back((to, output, *next_to));
                    }
                }
            }
        }
    }

    (low_count * high_count, 0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

// I feel like this can be done as one Vec<u8>.

struct Module<'a> {
    destinations: Vec<&'a [u8]>,
    brain: ModuleBrain<'a>,
}

enum ModuleBrain<'a> {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule<'a>),
}

struct FlipFlopModule {
    state: bool,
}

impl<'a> FlipFlopModule {
    fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        if let Pulse::Low = pulse {
            self.state = !self.state;
            if self.state {
                Some(Pulse::High)
            } else {
                Some(Pulse::Low)
            }
        } else {
            None
        }
    }
}

struct ConjunctionModule<'a> {
    inputs: HashMap<&'a [u8], Pulse>,
}

impl<'a> ConjunctionModule<'a> {
    fn init_input(&mut self, id: &'a [u8]) {
        self.inputs.insert(id, Pulse::Low);
    }

    fn process(&mut self, id: &'a [u8], pulse: Pulse) -> Pulse {
        self.inputs.insert(id, pulse);
        if self.inputs.values().all(|p| *p == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

fn parse_destinations(mut line: &[u8]) -> Vec<&[u8]> {
    let mut vec = Vec::new();

    while !line.is_empty() {
        if let Some(pos) = line.iter().position(|b| *b == b',') {
            vec.push(&line[..pos]);
            line = &line[pos + 2..];
        } else {
            vec.push(line);
            break;
        }
    }

    vec
}

#[cfg(test)]
#[test]
fn example_1() {
    let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let output = solve(input);
    assert_eq!(output.0, 32000000);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let output = solve(input);
    assert_eq!(output.0, 11687500);
}
