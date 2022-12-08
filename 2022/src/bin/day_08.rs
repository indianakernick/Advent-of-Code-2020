use advent_of_code_2022 as util;

fn main() {
    let mut rows = Vec::new();

    util::each_line("input/day_08.txt", |line| {
        rows.push(line
            .as_bytes()
            .iter()
            .map(|b| (*b, false, 1usize))
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

    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            for x1 in (0..x).rev() {
                if rows[y][x1].0 >= rows[y][x].0 || x1 == 0 {
                    rows[y][x].2 *= x - x1;
                    break;
                }
            }

            for x1 in x + 1..rows[y].len() {
                if rows[y][x1].0 >= rows[y][x].0 || x1 == rows[y].len() - 1 {
                    rows[y][x].2 *= x1 - x;
                    break;
                }
            }

            for y1 in (0..y).rev() {
                if rows[y1][x].0 >= rows[y][x].0 || y1 == 0 {
                    rows[y][x].2 *= y - y1;
                    break;
                }
            }

            for y1 in y + 1..rows.len() {
                if rows[y1][x].0 >= rows[y][x].0 || y1 == rows.len() - 1 {
                    rows[y][x].2 *= y1 - y;
                    break;
                }
            }
        }
    }

    let mut max = 0;
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].2 > max {
                max = rows[y][x].2;
            }
        }
    }
    println!("Part 2: {}", max);
}
