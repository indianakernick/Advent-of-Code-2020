pub fn lines_iter(str: &str) -> impl Iterator<Item = &[u8]> {
    str
        .as_bytes()
        .split_inclusive(|b| *b == b'\n')
        .map(|bytes| bytes.strip_suffix(&[b'\n']).unwrap_or(bytes))
}

pub fn index_of(bytes: &[u8], needle: u8) -> usize {
    bytes
        .iter()
        .position(|b| *b == needle)
        .unwrap()
}

pub fn index_of_after(bytes: &[u8], needle: u8, index: usize) -> usize {
    bytes[index..]
        .iter()
        .position(|b| *b == needle)
        .unwrap()
        + index
}

pub fn parse_u32(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b - b'0') as u32 * POW_10_32[i])
        .sum()
}

pub fn parse_i32(bytes: &[u8]) -> i32 {
    if bytes[0] == b'-' {
        let result = -(parse_u32(&bytes[1..]) as i32);
        debug_assert!(result < 0, "underflow");
        result
    } else {
        let result = parse_u32(bytes) as i32;
        debug_assert!(result >= 0, "overflow");
        result
    }
}

const POW_10_32: [u32; 10] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
];

pub fn parse_u64(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b - b'0') as u64 * POW_10_64[i])
        .sum()
}

const POW_10_64: [u64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];
