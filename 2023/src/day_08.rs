use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u64, u64) {
    let mut lines = common::lines_iter(input);

    let instructions = lines.next().unwrap();

    lines.next().unwrap();

    let mut network = Network::new();

    for line in lines {
        network.insert(
            [line[0], line[1], line[2]],
            ([line[7], line[8], line[9]], [line[12], line[13], line[14]])
        );
    }

    let network = &network;
    let nodes = network
        .keys()
        .filter(|node| node[2] == b'A')
        .copied()
        .collect::<Vec<_>>();

    std::thread::scope(|s| {
        let mut threads = Vec::new();

        threads.push(s.spawn(|| {
            if network.contains_key(b"AAA") {
                count_steps::<true>(network, instructions, *b"AAA")
            } else {
                0
            }
        }));

        for node in nodes {
            threads.push(s.spawn(move || {
                count_steps::<false>(network, instructions, node)
            }));
        }

        let mut iter = threads.into_iter();
        let count_1 = iter.next().unwrap().join().unwrap();
        let count_2 = iter.map(|t| t.join().unwrap()).reduce(common::lcm).unwrap();

        (count_1, count_2)
    })
}

type Node = [u8; 3];
type Network = HashMap<Node, (Node, Node)>;

fn count_steps<const FULL_END: bool>(
    network: &Network,
    instructions: &[u8],
    mut node: Node,
) -> u64 {
    let mut count = 0;

    'repeat: loop {
        for instr in instructions.iter() {
            let (left, right) = network.get(&node).unwrap();

            node = if *instr == b'L' { *left } else { *right };
            count += 1;

            if (FULL_END && node == *b"ZZZ") || (!FULL_END && node[2] == b'Z') {
                break 'repeat;
            }
        }
    }

    count
}

#[cfg(test)]
#[test]
fn example_1() {
    let input =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let output = solve(input);
    assert_eq!(output.0, 2);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let output = solve(input);
    assert_eq!(output.0, 6);
}

#[cfg(test)]
#[test]
fn example_3() {
    let input =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let output = solve(input);
    assert_eq!(output.1, 6);
}
