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
        let mut win_count = 0;

        for button_time in 1..*time {
            if button_time * (time - button_time) > *distance {
                win_count += 1;
            }
        }

        product *= win_count;
    }

    (product, 0)
}
