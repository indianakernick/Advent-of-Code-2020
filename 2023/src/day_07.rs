use std::{collections::HashMap, cmp::Ordering};

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut hands = Vec::new();

    for line in common::lines_iter(input) {
        hands.push((&line[..5], common::parse_u32(&line[6..])));
    }

    hands.sort_by(|(a, _), (b, _)| compare::<false>(a, b));

    let score_1 = score_sum(&hands);

    hands.sort_by(|(a, _), (b, _)| compare::<true>(a, b));

    let score_2 = score_sum(&hands);

    (score_1, score_2)
}

fn compare<const WILD: bool>(a_hand: &[u8], b_hand: &[u8]) -> Ordering {
    let a_type = classify::<WILD>(a_hand);
    let b_type = classify::<WILD>(b_hand);

    if a_type != b_type {
        return a_type.cmp(&b_type);
    }

    for (a_card, b_card) in a_hand.iter().zip(b_hand.iter()) {
        let a_strength = card_strength::<WILD>(*a_card);
        let b_strength = card_strength::<WILD>(*b_card);

        if a_strength != b_strength {
            return a_strength.cmp(&b_strength);
        }
    }

    Ordering::Equal
}

fn score_sum(hands: &Vec<(&[u8], u32)>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as u32 * bet)
        .sum()
}

fn classify<const WILD: bool>(hand: &[u8]) -> u32 {
    let mut map = HashMap::new();

    for card in hand.iter() {
        map.entry(*card).and_modify(|count| *count += 1).or_insert(1u32);
    }

    let mut counts = map.iter().map(|(card, count)| (*card, *count)).collect::<Vec<_>>();
    counts.sort_unstable_by_key(|(_, count)| *count);

    if WILD && counts.len() > 1 {
        let wild_index = counts.iter().position(|(card, _)| *card == b'J');
        if let Some(index) = wild_index {
            let count = counts.remove(index).1;
            let last = counts.len() - 1;
            counts[last].1 += count;
        }
    }

    if counts.len() == 1 {
        return 6; // five of a kind
    } else if counts.len() == 2 {
        if counts[0].1 == 1 {
            return 5; // four of a kind
        } else {
            return 4; // full house
        }
    } else if counts.len() == 3 {
        if counts[0].1 == 1 && counts[1].1 == 1 {
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

fn card_strength<const WILD: bool>(card: u8) -> usize {
    if WILD && card == b'J' {
        0
    } else {
        CARD_STRENGTH.iter().position(|c| *c == card).unwrap() + 1
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
