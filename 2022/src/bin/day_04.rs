use advent_of_code_2022 as util;
use text_io::read;

fn main() {
    let mut contain_count = 0u64;
    let mut overlap_count = 0u64;

    util::each_line("input/day_04.txt", |line| {
        let mut line_iter = line.bytes();
        let first_low: u32 = read!("{}-", line_iter);
        let first_high: u32 = read!("{},", line_iter);
        let second_low: u32 = read!("{}-", line_iter);
        let second_high: u32 = read!("{}", line_iter);

        if (first_low >= second_low && first_high <= second_high)
            || (second_low >= first_low && second_high <= first_high) {
            contain_count += 1;
        }

        if (first_low <= second_high && first_high >= second_low)
            || (second_low <= first_high && second_high >= first_low) {
            overlap_count += 1;
        }
    });

    println!("Part 1: {}", contain_count);
    println!("Part 2: {}", overlap_count);
}
