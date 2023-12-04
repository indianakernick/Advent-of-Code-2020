use std::collections::HashMap;

pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = (0, 0);
    let mut lines = input.lines();
    let mut prev = lines.next().unwrap().as_bytes();
    let mut curr = lines.next().unwrap().as_bytes();
    let mut next = lines.next().unwrap().as_bytes();
    let mut gears = HashMap::new();
    let mut row = 2;

    add_assign(&mut sum, search(prev, prev, curr, 0, &mut gears));
    add_assign(&mut sum, search(prev, curr, next, 1, &mut gears));

    while let Some(next_next) = lines.next() {
        prev = curr;
        curr = next;
        next = next_next.as_bytes();

        add_assign(&mut sum, search(prev, curr, next, row, &mut gears));
        row += 1;
    }

    add_assign(&mut sum, search(curr, next, next, row, &mut gears));

    sum
}

fn search(
    prev: &[u8],
    curr: &[u8],
    next: &[u8],
    row: usize,
    gears: &mut HashMap<(usize, usize), u32>
) -> (u32, u32) {
    let mut part_number_sum = 0;
    let mut gear_ratio_sum = 0;
    let mut is_part = false;
    let mut digit_index = 0;
    let mut part_number = 0;
    let mut gear_pos = None;

    for i in (0..curr.len()).rev() {
        if curr[i].is_ascii_digit() {
            if i + 1 < curr.len() {
                is_part |= is_symbol(prev[i + 1]);
                is_part |= is_symbol(curr[i + 1]);
                is_part |= is_symbol(next[i + 1]);

                if is_gear(prev[i + 1]) {
                    gear_pos = Some((row - 1, i + 1));
                }
                if is_gear(curr[i + 1]) {
                    gear_pos = Some((row, i + 1));
                }
                if is_gear(next[i + 1]) {
                    gear_pos = Some((row + 1, i + 1));
                }
            }

            is_part |= is_symbol(prev[i]);
            is_part |= is_symbol(next[i]);

            if is_gear(prev[i]) {
                gear_pos = Some((row - 1, i));
            }
            if is_gear(next[i]) {
                gear_pos = Some((row + 1, i));
            }

            part_number += (curr[i] - b'0') as u32 * 10u32.pow(digit_index);
            digit_index += 1;
        } else if digit_index > 0 {
            is_part |= is_symbol(prev[i]);
            is_part |= is_symbol(curr[i]);
            is_part |= is_symbol(next[i]);

            if is_gear(prev[i]) {
                gear_pos = Some((row - 1, i));
            }
            if is_gear(curr[i]) {
                gear_pos = Some((row, i));
            }
            if is_gear(next[i]) {
                gear_pos = Some((row + 1, i));
            }

            if let Some(pos) = gear_pos {
                if let Some(other_part_number) = gears.get(&pos) {
                    gear_ratio_sum += part_number * other_part_number;
                } else {
                    gears.insert(pos, part_number);
                }
            }

            if is_part {
                part_number_sum += part_number;
            }

            is_part = false;
            digit_index = 0;
            part_number = 0;
            gear_pos = None;
        }
    }

    if digit_index > 0 && is_part {
        part_number_sum += part_number;
    }

    if let Some(pos) = gear_pos {
        if let Some(other_part_number) = gears.get(&pos) {
            gear_ratio_sum += part_number * other_part_number;
        } else {
            gears.insert(pos, part_number);
        }
    }

    (part_number_sum, gear_ratio_sum)
}

fn is_symbol(ch: u8) -> bool {
    !ch.is_ascii_digit() && ch != b'.'
}

fn is_gear(ch: u8) -> bool {
    ch == b'*'
}

fn add_assign(lhs: &mut (u32, u32), rhs: (u32, u32)) {
    lhs.0 += rhs.0;
    lhs.1 += rhs.1;
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let output = solve(input);
    assert_eq!(output.0, 4361);
    assert_eq!(output.1, 467835);
}
