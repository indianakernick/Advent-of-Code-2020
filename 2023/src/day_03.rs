pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = 0;
    let mut lines = input.lines();
    let mut prev = lines.next().unwrap().as_bytes();
    let mut curr = lines.next().unwrap().as_bytes();
    let mut next = lines.next().unwrap().as_bytes();

    sum += search(prev, prev, curr);
    sum += search(prev, curr, next);

    while let Some(next_next) = lines.next() {
        prev = curr;
        curr = next;
        next = next_next.as_bytes();

        sum += search(prev, curr, next);
    }

    sum += search(curr, next, next);

    (sum, 0)
}

fn search(prev: &[u8], curr: &[u8], next: &[u8]) -> u32 {
    let mut sum = 0;
    let mut is_part = false;
    let mut digit_index = 0;
    let mut part_number = 0;

    for i in (0..curr.len()).rev() {
        if curr[i].is_ascii_digit() {
            if i + 1 < curr.len() {
                is_part |= is_symbol(prev[i + 1]);
                is_part |= is_symbol(curr[i + 1]);
                is_part |= is_symbol(next[i + 1]);
            }
            is_part |= is_symbol(prev[i]);
            is_part |= is_symbol(next[i]);

            part_number += (curr[i] - b'0') as u32 * 10u32.pow(digit_index);
            digit_index += 1;
        } else if digit_index > 0 {
            is_part |= is_symbol(prev[i]);
            is_part |= is_symbol(curr[i]);
            is_part |= is_symbol(next[i]);

            if is_part {
                sum += part_number;
            }

            is_part = false;
            digit_index = 0;
            part_number = 0;
        }
    }

    if digit_index > 0 && is_part {
        sum += part_number;
    }

    sum
}

fn is_symbol(ch: u8) -> bool {
    !ch.is_ascii_digit() && ch != b'.'
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
}
