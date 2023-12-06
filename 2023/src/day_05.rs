use crate::common;

pub fn solve(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap().as_bytes();
    let seeds = common::parse_delimited_list(&seeds_line[7..], b' ')
        .collect::<Vec<_>>();
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

    let mut min = (u64::MAX, 0);

    for seed in seeds.iter() {
        min.0 = min.0.min(map_value(*seed, &mappings));
    }

    'location: loop {
        let mut value = min.1;

        for mapping in mappings.iter().rev() {
            for range in mapping.iter() {
                if value >= range.destination_start && value < range.destination_start + range.length {
                    value = value - range.destination_start + range.source_start;
                    break;
                }
            }
        }

        for pair in seeds.chunks(2) {
            if value >= pair[0] && value < pair[0] + pair[1] {
                break 'location;
            }
        }

        min.1 += 1;
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

fn parse_range(bytes: &[u8]) -> MapRange {
    let mut iter = common::parse_delimited_list(bytes, b' ');
    MapRange {
        destination_start: iter.next().unwrap(),
        source_start: iter.next().unwrap(),
        length: iter.next().unwrap(),
    }
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
