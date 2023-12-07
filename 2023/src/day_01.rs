use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = (0, 0);

    for line in common::lines_iter(input) {
        sum.0 += first_last_sum::<false>(line);
        sum.1 += first_last_sum::<true>(line);
    }

    sum
}

const WORDS: [&[u8]; 9] = [
    b"one",
    b"two",
    b"three",
    b"four",
    b"five",
    b"six",
    b"seven",
    b"eight",
    b"nine",
];

fn first_last_sum<const USE_WORDS: bool>(line: &[u8]) -> u32 {
    let mut first = None;
    let mut last = None;

    for (char_idx, char) in line.iter().enumerate() {
        let mut digit = None;

        if char.is_ascii_digit() {
            digit = Some(char - b'0');
        } else if USE_WORDS {
            for (word_idx, word) in WORDS.iter().enumerate() {
                if line[char_idx..].starts_with(word) {
                    digit = Some(word_idx as u8 + 1);
                }
            }
        }

        if let Some(found) = digit {
            first.get_or_insert(found);
            last = digit;
        }
    }

    (first.unwrap_or(0) * 10 + last.unwrap_or(0)) as u32
}

#[cfg(test)]
#[test]
fn example_1() {
    let input =
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let output = solve(input);
    assert_eq!(output.0, 142);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let output = solve(input);
    assert_eq!(output.1, 281);
}
