use advent_of_code_2022 as util;

fn main() {
    let mut sum = 0u64;

    util::each_line("input/day_03.txt", |line| {
        let first = line[..line.len() / 2].as_bytes();
        let second = line[line.len() / 2..].as_bytes();

        for item in first.iter() {
            if second.contains(item) {
                if item.is_ascii_lowercase() {
                    sum += (item - b'a' + 1) as u64;
                } else {
                    sum += (item - b'A' + 27) as u64;
                }
                break;
            }
        }
    });

    println!("Part 1: {}", sum);
}
