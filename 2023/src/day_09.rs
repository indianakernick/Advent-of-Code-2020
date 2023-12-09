use crate::common;

pub fn solve(input: &str) -> (i32, i32) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for line in common::lines_iter(input) {
        let mut index = 0;
        let mut sequences = Vec::new();

        sequences.push(line
            .split(|b| *b == b' ')
            .map(|bytes| std::str::from_utf8(bytes).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>());

        while sequences[index].iter().any(|value| *value != 0) {
            let next_sequence = sequences[index]
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();

            sequences.push(next_sequence);
            index += 1;
        }

        for i in (0..sequences.len() - 1).rev() {
            let current = &sequences[i];
            let current_last = current[current.len() - 1];
            let previous = &sequences[i + 1];
            let previous_last = previous[previous.len() - 1];
            sequences[i].push(current_last + previous_last);
        }

        sum_1 += sequences[0].last().unwrap();

        for i in (0..sequences.len() - 1).rev() {
            let current = &sequences[i];
            let current_last = current[0];
            let previous = &sequences[i + 1];
            let previous_last = previous[0];
            sequences[i].insert(0, current_last - previous_last);
        }

        sum_2 += sequences[0][0];
    }

    (sum_1, sum_2)
}
