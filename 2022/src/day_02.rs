pub fn solve(input: &str) -> (u64, u64) {
    const LOSING_MOVE: [u8; 3] = [
        2, // Rock defeats Scissors
        0, // Paper defeats Rock
        1, // Scissors defeats Paper
    ];

    let mut score_1 = 0u64;
    let mut score_2 = 0u64;

    for line in input.lines() {
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
    }

    (score_1, score_2)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"A Y
B X
C Z";
    let output = solve(input);
    assert_eq!(output.0, 15);
    assert_eq!(output.1, 12);
}
