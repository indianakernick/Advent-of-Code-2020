use std::{collections::HashMap, cmp::Ordering};

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut hands = Vec::new();

    for line in common::lines_iter(input) {
        hands.push((&line[..5], common::parse_u32(&line[6..])));
    }

    hands.sort_by(|(a, _), (b, _)| {
        let a_type = classify(a);
        let b_type = classify(b);

        if a_type != b_type {
            return a_type.cmp(&b_type);
        }

        for (a, b) in a.iter().zip(b.iter()) {
            let a_strength = CARD_STRENGTH.iter().position(|card| card == a).unwrap();
            let b_strength = CARD_STRENGTH.iter().position(|card| card == b).unwrap();
            if a_strength != b_strength {
                return a_strength.cmp(&b_strength);
            }
        }

        Ordering::Equal
    });

    let sum = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as u32 * bet)
        .sum();

    (sum, 0)
}

fn classify(hand: &[u8]) -> u32 {
    let mut map = HashMap::new();

    for card in hand.iter() {
        map.entry(*card).and_modify(|count| *count += 1).or_insert(1u32);
    }

    let mut counts = map.iter().map(|(_, count)| *count).collect::<Vec<_>>();
    counts.sort_unstable();

    if counts.len() == 1 {
        return 6; // five of a kind
    } else if counts.len() == 2 {
        if counts[0] == 1 {
            return 5; // four of a kind
        } else {
            return 4; // full house
        }
    } else if counts.len() == 3 {
        if counts[0] == 1 && counts[1] == 1 {
            return 3; // three of a kind
        } else {
            return 2; // two pair
        }
    } else if counts.len() == 4 {
        return 1; // one pair
    } else {
        return 0;
    }
}

const CARD_STRENGTH: [u8; 13] = [
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b'T',
    b'J',
    b'Q',
    b'K',
    b'A',
];
