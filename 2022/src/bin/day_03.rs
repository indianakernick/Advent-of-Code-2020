use advent_of_code_2022 as util;

fn main() {
    let mut sum = 0u64;
    let mut found = Vec::new();

    util::each_line("input/day_03.txt", |line| {
        let first = line[..line.len() / 2].as_bytes();
        let second = line[line.len() / 2..].as_bytes();

        found.clear();

        for item in first.iter() {
            if !found.contains(item) && second.contains(item) {
                found.push(*item);
                if item.is_ascii_lowercase() {
                    sum += (item - b'a' + 1) as u64;
                } else if item.is_ascii_uppercase() {
                    sum += (item - b'A' + 27) as u64;
                }
            }
        }
    });

    println!("Part 1: {}", sum);
}
