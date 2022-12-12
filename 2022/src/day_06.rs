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
