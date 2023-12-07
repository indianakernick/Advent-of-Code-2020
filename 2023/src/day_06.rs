use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines = common::lines_iter(input);
    let mut races = Vec::new();

    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
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

#[cfg(test)]
#[test]
fn example() {
    let input =
"Time:      7  15   30
Distance:  9  40  200";
    let output = solve(input);
    assert_eq!(output.0, 288);
    assert_eq!(output.1, 71503);
}
