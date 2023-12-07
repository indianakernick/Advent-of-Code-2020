use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines = input.lines();
    let mut races = Vec::new();

    let time_line = lines.next().unwrap().as_bytes();
    let distance_line = lines.next().unwrap().as_bytes();
    let mut index = 0;

    while index < time_line.len() {
        let time_start = time_line[index..]
            .iter()
            .position(u8::is_ascii_digit)
            .unwrap()
            + index;
        let distance_start = distance_line[index..]
            .iter()
            .position(u8::is_ascii_digit)
            .unwrap()
            + index;

        let space = time_line[time_start..]
            .iter()
            .position(|b| *b == b' ')
            .map_or(time_line.len(), |i| i + time_start);

        races.push((
            common::parse_u32(&time_line[time_start..space]),
            common::parse_u32(&distance_line[distance_start..space]),
        ));

        index = space;
    }

    let mut product = 1;

    for (time, distance) in races.iter() {
        product *= (1..*time)
            .filter(|button_time| button_time * (time - button_time) > *distance)
            .count() as u32;
    }

    let time = common::parse_u64(time_line
        .iter()
        .copied()
        .filter(u8::is_ascii_digit)
        .collect::<Vec<_>>()
        .as_slice());
    let distance = common::parse_u64(distance_line
        .iter()
        .copied()
        .filter(u8::is_ascii_digit)
        .collect::<Vec<_>>()
        .as_slice());

    let win_count = (1..time)
        .filter(|button_time| button_time * (time - button_time) > distance)
        .count() as u32;

    (product, win_count)
}
