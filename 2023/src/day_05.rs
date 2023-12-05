pub fn solve(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap().as_bytes();
    let seeds = parse_number_list(&seeds_line[7..]);
    let mut mappings = Vec::new();

    lines.next().unwrap();

    'mapping: loop {
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
                break 'mapping;
            }
        }
    }

    let mut min = (u64::MAX, u64::MAX);

    for seed in seeds.iter() {
        min.0 = min.0.min(map_value(*seed, &mappings));
    }

    for pair in seeds.chunks(2) {
        for seed in pair[0]..pair[0] + pair[1] {
            min.1 = min.1.min(map_value(seed, &mappings));
        }
    }

    min
}

fn map_value(mut value: u64, mappings: &Vec<Vec<MapRange>>) -> u64 {
    for mapping in mappings.iter() {
        for range in mapping.iter() {
            if value >= range.source_start && value < range.source_start + range.length {
                value = value - range.source_start + range.destination_start;
                break;
            }
        }
    }
    value
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

struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let output = solve(input);
    assert_eq!(output.0, 35);
    assert_eq!(output.1, 46);
}
