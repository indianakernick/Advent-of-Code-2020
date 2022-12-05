use advent_of_code_2022 as util;
use text_io::read;

fn main() {
    let mut count = 0u64;

    util::each_line("input/day_04.txt", |line| {
        let mut line_iter = line.bytes();
        let first_low: u32 = read!("{}-", line_iter);
        let first_high: u32 = read!("{},", line_iter);
        let second_low: u32 = read!("{}-", line_iter);
        let second_high: u32 = read!("{}", line_iter);

        if (first_low >= second_low && first_high <= second_high)
            || (second_low >= first_low && second_high <= first_high) {
            count += 1;
        }
    });

    println!("Part 1: {}", count);
}
