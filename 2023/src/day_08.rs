use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines = common::lines_iter(input);

    let instructions = lines.next().unwrap();

    lines.next().unwrap();

    let mut network = HashMap::<&[u8], (&[u8], &[u8])>::new();

    for line in lines {
        network.insert(&line[..3], (&line[7..10], &line[12..15]));
    }

    let mut node: &[u8] = b"AAA";
    let mut count = 0;

    while node != b"ZZZ" {
        for instr in instructions.iter() {
            let (left, right) = network.get(node).unwrap();
            if *instr == b'L' {
                node = left;
            } else {
                node = right;
            }
            count += 1;
        }
    }

    (count, 0)
}
