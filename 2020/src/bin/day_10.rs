use adventofcode2020::*;
use std::collections::hash_map::HashMap;

fn product(adapters: &Vec<u64>) -> u64 {
    let mut diff_1 = 0;
    let mut diff_3 = 1;
    let mut prev = 0;
    for adapter in adapters {
        let diff = adapter - prev;
        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1;
        }
        prev = *adapter;
    }
    diff_1 * diff_3
}

fn permutations(cache: &mut HashMap<usize, u64>, adapters: &Vec<u64>, from: usize) -> u64 {
    if from == adapters.len() - 1 {
        return 1;
    }

    match cache.get(&from) {
        Some(value) => *value,
        None => {
            let prev = if from == usize::MAX { 0 } else { adapters[from] };
            let mut count = 0;
            for step in 1..4 {
                let next = from.wrapping_add(step);
                if next < adapters.len() && adapters[next] - prev <= 3 {
                    count += permutations(cache, adapters, next);
                }
            }
            cache.insert(from, count);
            count
        }
    }
}

fn main() {
    let mut adapters = line_iter_from_file("input/day_10.txt")
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    adapters.sort();

    println!("Part one: {}", product(&adapters));
    println!("Part two: {}", permutations(&mut HashMap::new(), &adapters, usize::MAX));
}
