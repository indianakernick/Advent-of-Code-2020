use advent_of_code_2022 as util;

fn main() {
    let mut max = 0u64;
    let mut curr = 0u64;

    util::each_line("input/day_01.txt", |line| {
        if line.len() == 0 {
            max = max.max(curr);
            curr = 0;
        } else {
            curr += u64::from_str_radix(line, 10).unwrap();
        }
    });

    println!("Part 1: {}", max);
}
