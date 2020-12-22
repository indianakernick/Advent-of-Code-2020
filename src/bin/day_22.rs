use adventofcode2020::*;
use std::collections::VecDeque;

fn parse_input() -> (VecDeque<u8>, VecDeque<u8>) {
    let mut line_iter = line_iter_from_file("input/day_22.txt");
    line_iter.next().unwrap(); // player_1:

    let player_1 = (&mut line_iter)
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    line_iter.next().unwrap(); // player_2:

    let player_2 = line_iter
        .map(|line| line.parse().unwrap())
        .collect();

    (player_1, player_2)
}

fn calculate_score(deck: &VecDeque<u8>) -> usize {
    let mut multiplier = deck.len();
    let mut score = 0;
    for card in deck.iter() {
        score += *card as usize * multiplier;
        multiplier -= 1;
    }
    score
}

fn combat(mut player_1: VecDeque<u8>, mut player_2: VecDeque<u8>) -> usize {
    loop {
        let top_1 = player_1.pop_front().unwrap();
        let top_2 = player_2.pop_front().unwrap();

        if top_1 > top_2 {
            player_1.push_back(top_1);
            player_1.push_back(top_2);
        } else if top_2 > top_1 {
            player_2.push_back(top_2);
            player_2.push_back(top_1);
        } else {
            panic!();
        }

        if player_1.is_empty() {
            return calculate_score(&player_2);
        } else if player_2.is_empty() {
            return calculate_score(&player_1);
        }
    }
}

#[derive(PartialEq)]
enum Winner {
    None,
    Player1,
    Player2
}

use std::sync::atomic::{AtomicUsize, Ordering};

fn recursive_combat_impl(level: usize, player_1: &mut VecDeque<u8>, player_2: &mut VecDeque<u8>) -> Winner {
    let mut rounds = Vec::<(VecDeque<u8>, VecDeque<u8>)>::new();

    loop {
        for round in rounds.iter() {
            if round.0 == *player_1 && round.1 == *player_2 {
                return Winner::Player1;
            }
        }

        rounds.push((player_1.clone(), player_2.clone()));

        let top_1 = player_1.pop_front().unwrap();
        let top_2 = player_2.pop_front().unwrap();

        let mut player = Winner::None;
        if player_1.len() >= top_1 as usize && player_2.len() >= top_2 as usize {
            player = recursive_combat_impl(
                level + 1,
                &mut player_1.iter().take(top_1 as usize).map(u8::clone).collect(),
                &mut player_2.iter().take(top_2 as usize).map(u8::clone).collect()
            );
        }

        if player == Winner::Player1 || top_1 > top_2 {
            player_1.push_back(top_1);
            player_1.push_back(top_2);
        } else if player == Winner::Player2 || top_2 > top_1 {
            player_2.push_back(top_2);
            player_2.push_back(top_1);
        } else {
            panic!();
        }

        if player_1.is_empty() {
            return Winner::Player2;
        } else if player_2.is_empty() {
            return Winner::Player1;
        }
    }
}

fn recursive_combat(mut player_1: VecDeque<u8>, mut player_2: VecDeque<u8>) -> usize {
    match recursive_combat_impl(0, &mut player_1, &mut player_2) {
        Winner::Player1 => calculate_score(&player_1),
        Winner::Player2 => calculate_score(&player_2),
        Winner::None => panic!(),
    }
}

fn main() {
    let (player_1, player_2) = parse_input();
    println!("Part one {}", combat(player_1.clone(), player_2.clone()));
    println!("Part two {}", recursive_combat(player_1, player_2));
}
