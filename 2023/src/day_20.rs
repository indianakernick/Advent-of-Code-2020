use std::collections::{hash_map::Entry, HashMap, VecDeque};

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut modules = parse_modules(input);

    populate_inputs(&mut modules);

    let mut tree = Vec::new();
    let mut existing = HashMap::<&[u8], u16>::new();

    build_module(&mut tree, &mut existing, &modules, b"");

    let mut low_count = 0;
    let mut high_count = 0;
    let mut queue = VecDeque::new();
    let mut solution = (0, 0);

    for i in 0.. {
        let final_pulse = process(&mut tree, &mut queue, &mut low_count, &mut high_count);

        if i == 999 {
            solution.0 = low_count * high_count;
            break;
        }

        // Still taking a long time.
        // if final_pulse == Some(Pulse::Low) {
        //     solution.1 = i + 1;
        //     break;
        // }
    }

    solution
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

struct Module<'a> {
    destinations: Vec<&'a [u8]>,
    brain: ModuleBrain<'a>,
}

enum ModuleBrain<'a> {
    Broadcaster,
    FlipFlop,
    Conjunction(Vec<&'a [u8]>),
}

fn parse_modules(input: &str) -> HashMap<&[u8], Module> {
    let mut modules = HashMap::new();

    for line in common::lines_iter(input) {
        if line[0] == b'%' {
            let arrow = common::index_of(line, b'-');
            modules.insert(&line[1..arrow - 1], Module {
                destinations: parse_destinations(&line[arrow + 3..]),
                brain: ModuleBrain::FlipFlop,
            });
        } else if line[0] == b'&' {
            let arrow = common::index_of(line, b'-');
            modules.insert(&line[1..arrow - 1], Module {
                destinations: parse_destinations(&line[arrow + 3..]),
                brain: ModuleBrain::Conjunction(Vec::new()),
            });
        } else if line.starts_with(b"broadcaster") {
            modules.insert(b"", Module {
                destinations: parse_destinations(&line[15..]),
                brain: ModuleBrain::Broadcaster,
            });
        } else {
            panic!("Invalid input");
        }
    }

    modules
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

fn populate_inputs<'a>(modules: &mut HashMap<&'a [u8], Module<'a>>) {
    let mut pairs = Vec::new();

    for (id, module) in modules.iter() {
        for dest in module.destinations.iter() {
            pairs.push((*id, *dest));
        }
    }

    for (from, to) in pairs {
        if let Some(to_module) = modules.get_mut(to) {
            if let ModuleBrain::Conjunction(b) = &mut to_module.brain {
                b.push(from);
            }
        }
    }
}

#[repr(u8)]
enum NodeType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

fn build_header(node_type: NodeType, destination_count: usize) -> u16 {
    ((node_type as u16) << 8) | destination_count as u16
}

fn deconstruct_header(header: u16) -> (NodeType, usize) {
    let node_type = match (header >> 8) as u8 {
        0 => NodeType::Broadcaster,
        1 => NodeType::FlipFlop,
        2 => NodeType::Conjunction,
        _ => unreachable!(),
    };
    (node_type, (header & 0xFF) as usize)
}

fn build_module<'a>(tree: &mut Vec<u16>, existing: &mut HashMap::<&'a [u8], u16>, modules: &'a HashMap<&'a [u8], Module>, id: &'a [u8]) -> u16 {
    let start = tree.len();

    match existing.entry(id) {
        Entry::Occupied(e) => return *e.get(),
        Entry::Vacant(e) => e.insert(start as u16),
    };

    let module = match modules.get(id) {
        Some(m) => m,
        None => return start as u16,
    };

    tree.push(0); // header
    tree.extend((0..module.destinations.len()).map(|_| 0));

    let node_type;

    match &module.brain {
        ModuleBrain::Broadcaster => {
            node_type = NodeType::Broadcaster;
        }
        ModuleBrain::FlipFlop => {
            node_type = NodeType::FlipFlop;
            tree.push(0); // state
        }
        ModuleBrain::Conjunction(input_ids) => {
            node_type = NodeType::Conjunction;
            tree.push(input_ids.len() as u16);
            let inputs = tree.len();
            tree.extend((0..input_ids.len()).map(|_| 0));

            for (i, input) in input_ids.iter().enumerate() {
                tree[inputs + i] = build_module(tree, existing, modules, input);
            }
        }
    }

    tree[start] = build_header(node_type, module.destinations.len());

    for (i, dest) in module.destinations.iter().enumerate() {
        tree[start + 1 + i] = build_module(tree, existing, modules, dest);
    }

    start as u16
}

fn process(
    tree: &mut [u16],
    queue: &mut VecDeque<(u16, Pulse, u16)>,
    low_count: &mut u32,
    high_count: &mut u32,
) -> Option<Pulse> {
    let mut final_pulse = None;

    queue.push_back((0, Pulse::Low, 0));

    while let Some((from, pulse, to)) = queue.pop_front() {
        match pulse {
            Pulse::Low => *low_count += 1,
            Pulse::High => *high_count += 1,
        }

        let to = to as usize;

        if to == tree.len() {
            final_pulse = Some(pulse);
            continue;
        }

        let (node_type, destinations) = deconstruct_header(tree[to]);

        let output = match node_type {
            NodeType::Broadcaster => Some(Pulse::Low),

            NodeType::FlipFlop => {
                if let Pulse::Low = pulse {
                    let state = &mut tree[to + 1 + destinations];
                    *state = !*state;
                    Some(if *state != 0 { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }

            NodeType::Conjunction => {
                let inputs_base = to + 1 + destinations + 1;
                let inputs_len = tree[inputs_base - 1] as usize;
                let mut all_high = true;

                for input in &mut tree[inputs_base..inputs_base + inputs_len] {
                    if *input & 0x7FFF == from {
                        match pulse {
                            Pulse::Low => *input &= 0x7FFF,
                            Pulse::High => *input |= 0x8000,
                        };
                    }
                    all_high = all_high && (*input & 0x8000 == 0x8000);
                }

                Some(if all_high { Pulse::Low } else { Pulse::High })
            }
        };

        if let Some(output) = output {
            for next_to in &tree[to + 1..to + 1 + destinations] {
                queue.push_back((to as u16, output, *next_to));
            }
        }
    }

    final_pulse
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
