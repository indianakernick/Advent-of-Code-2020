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
    let mut digits = Digits::default();
    let mut symbols = Symbols::default();

    let mut update_part_number_sum = |digits: &Digits, symbols: &Symbols| {
        if symbols.is_part {
            part_number_sum += digits.part_number;
        }
    };

    let mut update_gear_ratio_sum = |digits: &Digits, symbols: &Symbols| {
        if let Some(pos) = symbols.gear_pos {
            let part_number = digits.part_number;
            if let Some(other_part_number) = gears.get(&pos) {
                gear_ratio_sum += part_number * other_part_number;
            } else {
                gears.insert(pos, part_number);
            }
        }
    };

    for i in (0..curr.len()).rev() {
        if curr[i].is_ascii_digit() {
            if i + 1 < curr.len() {
                symbols.handle_char(prev, row.wrapping_sub(1), i + 1);
                symbols.handle_char(curr, row, i + 1);
                symbols.handle_char(next, row + 1, i + 1);
            }

            symbols.handle_char(prev, row.wrapping_sub(1), i);
            symbols.handle_char(next, row + 1, i);

            digits.handle_char(curr[i]);
        } else if digits.has_digits() {
            symbols.handle_char(prev, row.wrapping_sub(1), i);
            symbols.handle_char(curr, row, i);
            symbols.handle_char(next, row + 1, i);

            update_part_number_sum(&digits, &symbols);
            update_gear_ratio_sum(&digits, &symbols);

            digits = Digits::default();
            symbols = Symbols::default();
        }
    }

    update_part_number_sum(&digits, &symbols);
    update_gear_ratio_sum(&digits, &symbols);

    (part_number_sum, gear_ratio_sum)
}

#[derive(Default)]
struct Digits {
    part_number: u32,
    digit_index: u32,
}

impl Digits {
    fn handle_char(&mut self, ch: u8) {
        self.part_number += (ch - b'0') as u32 * 10u32.pow(self.digit_index);
        self.digit_index += 1;
    }

    fn has_digits(&self) -> bool {
        self.digit_index > 0
    }
}

#[derive(Default)]
struct Symbols {
    is_part: bool,
    gear_pos: Option<(usize, usize)>,
}

impl Symbols {
    fn handle_char(&mut self, line: &[u8], row: usize, col: usize) {
        let ch = line[col];
        if Self::is_gear(ch) {
            self.gear_pos = Some((row, col));
            self.is_part = true;
        } else {
            self.is_part |= Self::is_symbol(ch);
        }
    }

    fn is_symbol(ch: u8) -> bool {
        !ch.is_ascii_digit() && ch != b'.'
    }

    fn is_gear(ch: u8) -> bool {
        ch == b'*'
    }
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
