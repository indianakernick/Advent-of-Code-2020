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

pub fn parse_delimited_list<'a>(bytes: &'a [u8], delimiter: u8) -> DelimitedListIterator<'a> {
    DelimitedListIterator { bytes, index: 0, number_start: 0, delimiter }
}

pub struct DelimitedListIterator<'a> {
    bytes: &'a [u8],
    index: usize,
    number_start: usize,
    delimiter: u8,
}

impl<'a> Iterator for DelimitedListIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.bytes.len() {
            if self.bytes[self.index] == self.delimiter {
                let number = &self.bytes[self.number_start..self.index];
                self.index += 1;
                self.number_start = self.index;
                return Some(parse_u64(number));
            }
            self.index += 1;
        }

        if self.number_start < self.bytes.len() {
            let number = &self.bytes[self.number_start..self.index];
            self.number_start = self.bytes.len();
            return Some(parse_u64(number));
        }

        None
    }
}
