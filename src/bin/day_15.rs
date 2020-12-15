use std::collections::hash_map::{HashMap, Entry};

fn find_spoken_number(start: &[u32], last_turn: u32) -> u32 {
    let mut numbers = HashMap::<u32, (u32, u32)>::new();
    for i in 0..start.len() {
        let turn = i as u32;
        numbers.insert(start[i], (turn, turn));
    }
    let mut last = start[start.len() - 1];

    for t in (start.len() as u32)..last_turn {
        match numbers.entry(last) {
            Entry::Occupied(entry) => {
                let pair = entry.get();
                last = pair.1 - pair.0;
                match numbers.entry(last) {
                    Entry::Occupied(mut entry) => {
                        let pair = entry.get_mut();
                        pair.0 = pair.1;
                        pair.1 = t;
                    },
                    Entry::Vacant(entry) => {
                        entry.insert((t, t));
                    }
                }
            },
            Entry::Vacant(entry) => {
                entry.insert((t, t));
            }
        }
    }

    last
}

fn main() {
    let start = [9, 12, 1, 4, 17, 0, 18];
    println!("Part one: {}", find_spoken_number(&start, 2020));
    println!("Part two: {}", find_spoken_number(&start, 30000000));
}
