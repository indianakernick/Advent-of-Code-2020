use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u32, u64) {
    let mut lines = common::lines_iter(input);

    let instructions = lines.next().unwrap();

    lines.next().unwrap();

    let mut network = HashMap::<[u8; 3], ([u8; 3], [u8; 3])>::new();

    for line in lines {
        network.insert(
            [line[0], line[1], line[2]],
            ([line[7], line[8], line[9]], [line[12], line[13], line[14]])
        );
    }

    let mut node_1 = b"AAA";
    let mut count_1 = 0;

    if network.contains_key(node_1) {
        while node_1 != b"ZZZ" {
            for instr in instructions.iter() {
                let (left, right) = network.get(node_1).unwrap();
                node_1 = if *instr == b'L' {
                    left
                } else {
                    right
                };
                count_1 += 1;
            }
        }
    }

    let nodes_2 = network
        .keys()
        .filter(|node| node[2] == b'A')
        .copied()
        .collect::<Vec<_>>();

    let count_2 = std::thread::scope(|s| {
        let mut threads = Vec::new();

        for i in 0..nodes_2.len() {
            let mut node = nodes_2[i];
            let network = &network;

            threads.push(s.spawn(move || {
                let mut count = 0u32;

                'repeat: loop {
                    for instr in instructions.iter() {
                        let (left, right) = network.get(&node).unwrap();

                        node = if *instr == b'L' {
                            *left
                        } else {
                            *right
                        };

                        count += 1;

                        if node[2] == b'Z' {
                            break 'repeat;
                        }
                    }
                }

                count
            }));
        }

        let mut lcm = 1;

        for thread in threads.into_iter() {
            let count = thread.join().unwrap() as u64;
            lcm = lcm * count / gcd(lcm, count);
        }

        lcm
    });

    (count_1, count_2)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
