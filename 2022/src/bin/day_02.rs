use advent_of_code_2022 as util;

fn main() {
    let mut score = 0u64;

    util::each_line("input/day_02.txt", |line| {
        let opponent = line.as_bytes()[0] - b'A';
        let me = line.as_bytes()[2] - b'X';

        score += me as u64 + 1;

        if opponent == me {
            score += 3;
        } else if (me == 0 && opponent == 2) || (me == 2 && opponent == 1) || (me == 1 && opponent == 0) {
            score += 6;
        }
    });

    println!("Part 1: {}", score);
}
