use crate::common;

pub fn solve(input: &str) -> (i32, i32) {
    let mut sum = (0, 0);
    let mut sequences = Vec::new();

    for line in common::lines_iter(input) {
        let mut index = 0;

        sequences.clear();

        sequences.push(line
            .split(|b| *b == b' ')
            .map(common::parse_i32)
            .collect::<Vec<_>>());

        loop {
            let next_sequence = sequences[index]
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();

            if next_sequence.iter().all(|value| *value == 0) {
                break;
            }

            sequences.push(next_sequence);
            index += 1;
        }

        let mut value = (0, 0);

        for i in (0..sequences.len()).rev() {
            let sequence = &sequences[i];
            value.0 = sequence[sequence.len() - 1] + value.0;
            value.1 = sequence[0] - value.1;
        }

        common::add_assign(&mut sum, value);
    }

    sum
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let output = solve(input);
    assert_eq!(output.0, 114);
    assert_eq!(output.1, 2);
}
