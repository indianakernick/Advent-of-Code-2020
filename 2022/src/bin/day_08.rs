use advent_of_code_2022 as util;

fn main() {
    let mut rows = Vec::new();

    util::each_line("input/day_08.txt", |line| {
        rows.push(line
            .as_bytes()
            .iter()
            .map(|b| (*b, false))
            .collect::<Vec<_>>());
    });

    for y in 0..rows.len() {
        let mut highest = 0u8;
        for x in 0..rows[y].len() {
            if rows[y][x].0 > highest {
                rows[y][x].1 = true;
                highest = rows[y][x].0;
            }
        }

        let mut highest = 0u8;
        for x in (0..rows[y].len()).rev() {
            if rows[y][x].0 > highest {
                rows[y][x].1 = true;
                highest = rows[y][x].0;
            }
        }
    }

    for x in 0..rows[0].len() {
        let mut highest = 0u8;
        for y in 0..rows.len() {
            if rows[y][x].0 > highest {
                rows[y][x].1 = true;
                highest = rows[y][x].0;
            }
        }

        let mut highest = 0u8;
        for y in (0..rows.len()).rev() {
            if rows[y][x].0 > highest {
                rows[y][x].1 = true;
                highest = rows[y][x].0;
            }
        }
    }

    let mut count = 0;
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].1 {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
}
