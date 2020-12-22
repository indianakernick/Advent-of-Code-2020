use std::collections::VecDeque;

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

fn recursive_combat_impl(level: usize, mut player_1: VecDeque<u8>, mut player_2: VecDeque<u8>) -> usize {
    let mut rounds = Vec::<(VecDeque<u8>, VecDeque<u8>)>::new();

    loop {
        for round in rounds.iter() {
            if round.0 == player_1 && round.1 == player_2 {
                if level == 0 {
                    return calculate_score(&player_1);
                } else {
                    return 1;
                }
            }
        }

        rounds.push((player_1.clone(), player_2.clone()));

        let top_1 = player_1.pop_front().unwrap();
        let top_2 = player_2.pop_front().unwrap();

        let mut player = 0;
        if player_1.len() >= top_1 as usize && player_2.len() >= top_2 as usize {
            player = recursive_combat_impl(
                level + 1,
                player_1.iter().take(top_1 as usize).map(u8::clone).collect(),
                player_2.iter().take(top_2 as usize).map(u8::clone).collect()
            );
        }

        if player == 1 || top_1 > top_2 {
            player_1.push_back(top_1);
            player_1.push_back(top_2);
        } else if player == 2 || top_2 > top_1 {
            player_2.push_back(top_2);
            player_2.push_back(top_1);
        } else {
            panic!();
        }

        if player_1.is_empty() {
            if level == 0 {
                return calculate_score(&player_2)
            } else {
                return 2;
            }
        } else if player_2.is_empty() {
            if level == 0 {
                return calculate_score(&player_1);
            } else {
                return 1;
            }
        }
    }
}

fn recursive_combat(player_1: VecDeque<u8>, player_2: VecDeque<u8>) -> usize {
    recursive_combat_impl(0, player_1, player_2)
}

fn main() {
    let player_1 = VecDeque::from(vec![
        14, 6, 21, 10, 1, 33, 7, 13, 25, 8, 17, 11, 28, 27, 50, 2, 35, 49, 19, 46, 3, 38, 23, 5, 43,
    ]);
    let player_2 = VecDeque::from(vec![
        18, 9, 12, 39, 48, 24, 32, 45, 47, 41, 40, 15, 22, 36, 30, 26, 42, 34, 20, 16, 4, 31, 37, 44, 29,
    ]);

    println!("Part one {}", combat(player_1.clone(), player_2.clone()));
    println!("Part two {}", recursive_combat(player_1, player_2));
}