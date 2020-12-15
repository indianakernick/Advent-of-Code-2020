use std::collections::hash_map::{HashMap, Entry};

fn find_spoken_number(start: &[u32], last_turn: u32) -> u32 {
    let mut numbers = HashMap::<u32, (u32, u32)>::new();
    for i in 0..start.len() {
        let turn = i as u32;
        numbers.insert(start[i], (turn, turn));
    }
    let mut last = start[start.len() - 1];

    for t in (start.len() as u32)..last_turn {
        let pair = numbers[&last];
        last = pair.1 - pair.0;
        let pair = numbers.entry(last).or_insert((t, t));
        pair.0 = pair.1;
        pair.1 = t;
    }

    last
}

fn main() {
    let start = [9, 12, 1, 4, 17, 0, 18];
    println!("Part one: {}", find_spoken_number(&start, 2020));
    println!("Part two: {}", find_spoken_number(&start, 30000000));
}
