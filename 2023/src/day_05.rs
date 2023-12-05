pub fn solve(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap().as_bytes();
    let seeds = parse_number_list(&seeds_line[7..]);
    let mut mappings = Vec::new();

    lines.next().unwrap();

    'outer: loop {
        let mut ranges = Vec::new();

        lines.next().unwrap();

        loop {
            if let Some(range_line) = lines.next() {
                if range_line.len() > 0 {
                    ranges.push(parse_range(range_line.as_bytes()));
                } else {
                    mappings.push(ranges);
                    break;
                }
            } else {
                mappings.push(ranges);
                break 'outer;
            }
        }
    }

    let mut min_location = u64::MAX;

    for seed in seeds.iter() {
        let mut value = *seed;

        for mapping in mappings.iter() {
            for range in mapping.iter() {
                if value >= range.source_start && value < range.source_start + range.length {
                    value = value - range.source_start + range.destination_start;
                    break;
                }
            }
        }

        min_location = min_location.min(value);
    }

    (min_location, 0)
}

fn parse_number_list(s: &[u8]) -> Vec<u64> {
    let mut list = Vec::new();

    let mut index = 0;
    let mut number_start = 0;

    while index < s.len() {
        if s[index] == b' ' {
            list.push(parse_number(&s[number_start..index]));
            number_start = index + 1;
        }

        index += 1;
    }

    list.push(parse_number(&s[number_start..index]));

    list
}

fn parse_range(s: &[u8]) -> MapRange {
    let vec = parse_number_list(s);
    MapRange {
        destination_start: vec[0],
        source_start: vec[1],
        length: vec[2],
    }
}

fn parse_number(s: &[u8]) -> u64 {
    s
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b - b'0') as u64 * 10u64.pow(i as u32))
        .sum()
}

#[derive(Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}
