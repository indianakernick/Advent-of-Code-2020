use text_io::scan;

fn manhattan(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut sensors = Vec::<((i32, i32), (i32, i32))>::new();

    for line in input.lines() {
        let sx: i32;
        let sy: i32;
        let bx: i32;
        let by: i32;
        scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);
        sensors.push(((sx, sy), (bx, by)));
    }

    let mut count = 0;
    let min_sensor_x = *sensors.iter().map(|((sx, _), _)| sx).min().unwrap();
    let max_sensor_x = *sensors.iter().map(|((sx, _), _)| sx).max().unwrap();
    let max_sensor_range = sensors.iter().map(|(s, b)| manhattan(*s, *b)).max().unwrap() as i32;
    const Y: i32 = 2000000;

    for x in min_sensor_x - max_sensor_range..=max_sensor_x + max_sensor_range {
        let mut within_sensor = false;
        let mut within_beacon = false;

        for (s, b) in sensors.iter() {
            if manhattan(*s, (x, Y)) <= manhattan(*s, *b) {
                within_sensor = true;
            }
            if x == b.0 && Y == b.1 {
                within_beacon = true;
            }
        }

        if within_sensor && !within_beacon {
            count += 1;
        }
    }

    (count, 0)
}
