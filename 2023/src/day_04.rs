use std::collections::HashSet;

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut points_sum = 0;
    let mut winning_set = HashSet::new();
    let mut card_copy_counts = Vec::<u32>::new();
    let mut card_index = 0;

    for line in input.lines() {
        let winning_count = count_winners(line.as_bytes(), &mut winning_set);

        if winning_count != 0 {
            points_sum += 1 << (winning_count - 1);
        }

        let new_length = card_index + winning_count + 1;

        if new_length > card_copy_counts.len() {
            card_copy_counts.reserve(new_length - card_copy_counts.len());
        }

        let total_count = if card_index < card_copy_counts.len() {
            1 + card_copy_counts[card_index]
        } else {
            1
        };

        for card in 0..winning_count {
            let copy_index = card_index + card + 1;

            if copy_index < card_copy_counts.len() {
                card_copy_counts[copy_index] += total_count;
                continue;
            }

            if copy_index > card_copy_counts.len() {
                card_copy_counts.push(0);
            }

            card_copy_counts.push(total_count);
        }

        card_index += 1;
    }

    (points_sum, card_copy_counts.iter().sum::<u32>() + card_index as u32)
}

fn count_winners(line: &[u8], winning_set: &mut HashSet<u8>) -> usize {
    let colon = common::index_of(line, b':');
    let mut index = colon + 2;

    while line[index] != b'|' {
        winning_set.insert(parse_value(&line[index..]));
        index += 3;
    }

    index += 2;

    let mut winning_count = 0;

    while index < line.len() {
        if winning_set.contains(&parse_value(&line[index..])) {
            winning_count += 1;
        }

        index += 3;
    }

    winning_set.clear();

    winning_count
}

fn parse_value(s: &[u8]) -> u8 {
    let mut value = 0;

    if s[0] != b' ' {
        value += 10 * (s[0] - b'0');
    }

    value + (s[1] - b'0')
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let output = solve(input);
    assert_eq!(output.0, 13);
    assert_eq!(output.1, 30);
}
