use std::collections::HashSet;

use advent_of_code_2022 as util;

fn main() {
    let stream = std::fs::read_to_string("input/day_06.txt").unwrap();
    let stream = stream.as_bytes();

    for i in 3..stream.len() {
        let mut chars = HashSet::new();
        chars.insert(stream[i - 3]);
        chars.insert(stream[i - 2]);
        chars.insert(stream[i - 1]);
        chars.insert(stream[i]);
        if chars.len() == 4 {
            println!("Part 1: {}", i + 1);
            break;
        }
    }
}
