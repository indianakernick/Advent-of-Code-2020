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

fn main() {
    let stream = std::fs::read_to_string("input/day_06.txt").unwrap();
    let stream = stream.as_bytes();

    println!("Part 1: {}", first_unique_sequence::<4>(stream));
    println!("Part 2: {}", first_unique_sequence::<14>(stream));
}
