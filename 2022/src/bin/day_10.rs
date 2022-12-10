use advent_of_code_2022 as util;

fn main() {
    let mut reg = 1;
    let mut cycle = 0;
    let mut strength = 0;

    let mut display = Vec::<Vec<u8>>::new();
    display.resize_with(6, || vec![b'.'; 40]);

    util::each_line("input/day_10.txt", |line| {
        if line == "noop" {
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
            let y = (cycle - 1) / 40;
            let x = (cycle - 1) % 40;
            if x == reg || x == reg - 1 || x == reg + 1 {
                display[y as usize][x as usize] = b'#';
            }
        } else if let Some(num) = line.strip_prefix("addx ") {
            let num: i32 = num.parse().unwrap();
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
            let y = (cycle - 1) / 40;
            let x = (cycle - 1) % 40;
            if x == reg || x == reg - 1 || x == reg + 1 {
                display[y as usize][x as usize] = b'#';
            }
            cycle += 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strength += cycle * reg;
            }
            let y = (cycle - 1) / 40;
            let x = (cycle - 1) % 40;
            if x == reg || x == reg - 1 || x == reg + 1 {
                display[y as usize][x as usize] = b'#';
            }
            reg += num;
        }
    });

    println!("Part 1: {}", strength);

    println!("Part 2:");
    for row in display.iter() {
        for col in row.iter() {
            print!("{}", *col as char);
        }
        println!();
    }
}
