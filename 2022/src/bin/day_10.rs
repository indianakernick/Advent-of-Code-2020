use advent_of_code_2022 as util;

fn main() {
    let mut reg = 1;
    let mut cycle = 0;
    let mut strength = 0;

    util::each_line("input/day_10.txt", |line| {
        if line == "noop" {
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
        } else if let Some(num) = line.strip_prefix("addx ") {
            let num: i32 = num.parse().unwrap();
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
            reg += num;
        }
    });

    println!("Part 1: {}", strength);
}
