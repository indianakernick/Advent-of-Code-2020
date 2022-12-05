use advent_of_code_2022 as util;

fn main() {
    const LOSING_MOVE: [u8; 3] = [
        2, // Rock defeats Scissors
        0, // Paper defeats Rock
        1, // Scissors defeats Paper
    ];

    let mut score_1 = 0u64;
    let mut score_2 = 0u64;

    util::each_line("input/day_02.txt", |line| {
        let opponent = line.as_bytes()[0] - b'A';
        let me = line.as_bytes()[2] - b'X';

        score_1 += me as u64 + 1;

        if opponent == me {
            score_1 += 3;
        } else if LOSING_MOVE[me as usize] == opponent {
            score_1 += 6;
        }

        let outcome = me;

        score_2 += outcome as u64 * 3;

        if outcome == 0 {
            score_2 += LOSING_MOVE[opponent as usize] as u64 + 1;
        } else if outcome == 1 {
            score_2 += opponent as u64 + 1;
        } else if outcome == 2 {
            score_2 += LOSING_MOVE[LOSING_MOVE[opponent as usize] as usize] as u64 + 1;
        }
    });

    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);
}
