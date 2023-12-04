use std::collections::HashSet;

pub fn solve(input: &str) -> (u32, u32) {
    let mut points_sum = 0;
    let mut winning_set = HashSet::new();
    let mut copy_card_counts = Vec::<u32>::new();
    let mut original_card_count = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();

        let colon = bytes.iter().position(|b| *b == b':').unwrap();
        let mut index = colon + 2;

        while bytes[index] != b'|' {
            winning_set.insert(get_value(&bytes[index..]));
            index += 3;
        }

        index += 2;

        let mut winning_count = 0;

        while index < bytes.len() {
            if winning_set.contains(&get_value(&bytes[index..])) {
                winning_count += 1;
            }

            index += 3;
        }

        winning_set.clear();

        if winning_count != 0 {
            points_sum += 1 << (winning_count - 1);
        }

        let new_length = original_card_count + winning_count + 1;

        if new_length > copy_card_counts.len() {
            copy_card_counts.reserve(new_length - copy_card_counts.len());
        }

        let multiplier = if original_card_count < copy_card_counts.len() {
            1 + copy_card_counts[original_card_count]
        } else {
            1
        };

        for card in 0..winning_count {
            let card_index = original_card_count + card + 1;

            if card_index < copy_card_counts.len() {
                copy_card_counts[card_index] += multiplier;
                continue;
            }
            if card_index > copy_card_counts.len() {
                copy_card_counts.push(0);
            }
            copy_card_counts.push(multiplier);
        }

        original_card_count += 1;
    }

    (points_sum, copy_card_counts.iter().sum::<u32>() + original_card_count as u32)
}

fn get_value(s: &[u8]) -> u8 {
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