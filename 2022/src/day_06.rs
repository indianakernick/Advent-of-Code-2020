fn first_unique_sequence<const LEN: usize>(stream: &[u8]) -> usize {
    for i in LEN - 1..stream.len() {
        let mut set = 0u32;
        for j in 0..LEN {
            set |= 1 << (stream[i - j] - b'a');
        }
        if set.count_ones() == LEN as u32 {
            return i + 1;
        }
    }

    0
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.as_bytes();

    (
        first_unique_sequence::<4>(input),
        first_unique_sequence::<14>(input),
    )
}

#[cfg(test)]
#[test]
fn example_1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let output = solve(input);
    assert_eq!(output.0, 7);
    assert_eq!(output.1, 19);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let output = solve(input);
    assert_eq!(output.0, 5);
    assert_eq!(output.1, 23);
}

#[cfg(test)]
#[test]
fn example_3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let output = solve(input);
    assert_eq!(output.0, 6);
    assert_eq!(output.1, 23);
}

#[cfg(test)]
#[test]
fn example_4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let output = solve(input);
    assert_eq!(output.0, 10);
    assert_eq!(output.1, 29);
}

#[cfg(test)]
#[test]
fn example_5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let output = solve(input);
    assert_eq!(output.0, 11);
    assert_eq!(output.1, 26);
}
