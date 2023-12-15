pub fn solve(input: &str) -> (u32, u32) {
    let sum = input[..input.len() - 1]
        .as_bytes()
        .split(|b| *b == b',')
        .map(|bytes| hash(bytes) as u32)
        .sum();

    (sum, 0)
}

fn hash(bytes: &[u8]) -> u8 {
    let mut value = 0u8;

    for b in bytes {
        value = value.overflowing_add(*b).0.overflowing_mul(17).0;
    }

    value
}
