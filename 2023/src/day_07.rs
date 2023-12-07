use std::cmp::Ordering;

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut hands = common::lines_iter(input)
        .map(|line| (
            Hand::try_from(&line[..HAND_SIZE]).unwrap(),
            common::parse_u32(&line[6..])
        ))
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|(a, _), (b, _)| compare::<false>(a, b));
    let score_1 = score_sum(&hands);

    hands.sort_unstable_by(|(a, _), (b, _)| compare::<true>(a, b));
    let score_2 = score_sum(&hands);

    (score_1, score_2)
}

const HAND_SIZE: usize = 5;
type Hand<'a> = &'a [u8; HAND_SIZE];

fn score_sum(hands: &Vec<(Hand, u32)>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as u32 * bet)
        .sum()
}

fn compare<const WILD: bool>(a_hand: Hand, b_hand: Hand) -> Ordering {
    let a_type = hand_type::<WILD>(a_hand);
    let b_type = hand_type::<WILD>(b_hand);
    let cmp_type = a_type.cmp(&b_type);

    if !cmp_type.is_eq() {
        return cmp_type;
    }

    for (a_card, b_card) in a_hand.iter().zip(b_hand.iter()) {
        let a_strength = card_strength::<WILD>(*a_card);
        let b_strength = card_strength::<WILD>(*b_card);
        let cmp_strength = a_strength.cmp(&b_strength);

        if !cmp_strength.is_eq() {
            return cmp_strength;
        }
    }

    Ordering::Equal
}

fn hand_type<const WILD: bool>(hand: Hand) -> HandType {
    let mut card_counts: [(u8, u8); HAND_SIZE] = [(0, 0); HAND_SIZE];
    let mut length = 0;

    for hand_card in hand.iter() {
        for (card, count) in card_counts.iter_mut() {
            if *card == 0 {
                *card = *hand_card;
                *count = 1;
                length += 1;
                break;
            } else if *card == *hand_card {
                *count += 1;
                break;
            }
        }
    }

    card_counts[0..length].sort_unstable_by_key(|(_, count)| *count);

    if WILD && length > 1 {
        let index = card_counts.iter().position(|(card, _)| *card == WILD_CARD);

        if let Some(index) = index {
            length -= 1;
            card_counts[length].1 += card_counts[index].1;

            if index < length {
                card_counts.copy_within(index + 1..length + 1, index);
            }
        }
    }

    if length == 1 {
        HandType::FiveOfAKind
    } else if length == 2 {
        if card_counts[0].1 == 1 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else if length == 3 {
        if card_counts[0].1 == 1 && card_counts[1].1 == 1 {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if length == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_strength<const WILD: bool>(card: u8) -> u8 {
    if WILD && card == WILD_CARD {
        0
    } else {
        CARD_STRENGTH.iter().position(|c| *c == card).unwrap() as u8 + 1
    }
}

const WILD_CARD: u8 = b'J';

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

#[cfg(test)]
#[test]
fn example() {
    let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let output = solve(input);
    assert_eq!(output.0, 6440);
    assert_eq!(output.1, 5905);
}
