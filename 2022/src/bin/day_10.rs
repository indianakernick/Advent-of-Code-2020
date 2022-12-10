use advent_of_code_2022 as util;

fn run_cycle(reg: i32, cycle: &mut i32, strength: &mut i32, display: &mut [u8]) {
    let x = *cycle % 40;
    if x == reg || x == reg - 1 || x == reg + 1 {
        display[*cycle as usize] = b'#';
    }
    *cycle += 1;
    if *cycle >= 20 && (*cycle - 20) % 40 == 0 {
        *strength += *cycle * reg;
    }
}

fn main() {
    let mut reg = 1;
    let mut cycle = 0;
    let mut strength = 0;
    let mut display = vec![b'.'; 6 * 40];

    util::each_line("input/day_10.txt", |line| {
        if line == "noop" {
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
        } else if let Some(num) = line.strip_prefix("addx ") {
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
            reg += num.parse::<i32>().unwrap();
        }
    });

    println!("Part 1: {}", strength);
    println!("Part 2:");
    for row in display.chunks(40) {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}
