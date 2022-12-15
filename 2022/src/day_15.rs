use text_io::scan;

fn manhattan(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solve_impl(input: &str, search_range: i32) -> (usize, usize) {
    let mut sensors = Vec::<((i32, i32), u32)>::new();
    let mut beacons = Vec::<(i32, i32)>::new();
    let mut min_sensor_x = i32::MAX;
    let mut max_sensor_x = i32::MIN;
    let mut max_sensor_range = 0;

    for line in input.lines() {
        let sx: i32;
        let sy: i32;
        let bx: i32;
        let by: i32;
        scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);

        let s = (sx, sy);
        let b = (bx, by);
        let range = manhattan(s, b);

        sensors.push((s, range));
        if !beacons.contains(&b) {
            beacons.push(b);
        }

        min_sensor_x = min_sensor_x.min(sx);
        max_sensor_x = max_sensor_x.max(sx);
        max_sensor_range = max_sensor_range.max(range as i32);
    }

    let mut covered_count = 0;
    let y = search_range / 2;
    let mut x = min_sensor_x - max_sensor_range;

    while x <= max_sensor_x + max_sensor_range {
        let mut within_sensor = None;

        for (s, range) in sensors.iter() {
            if manhattan(*s, (x, y)) <= *range {
                within_sensor = Some(s.0 + (range - y.abs_diff(s.1)) as i32 + 1);
                break;
            }
        }

        if let Some(next_x) = within_sensor {
            let mut beacon_count = 0;

            for b in beacons.iter() {
                if b.1 == y && x <= b.0 && b.0 < next_x {
                    beacon_count += 1;
                }
            }

            covered_count += (next_x - x) as usize - beacon_count;
            x = next_x;
        } else {
            x += 1;
        }
    }

    let thread_count = std::thread::available_parallelism().unwrap().get();

    let tuning_freq = std::thread::scope(|s| {
        let mut threads = Vec::new();
        let sensors = &sensors;
        let search_range = search_range + 1;

        for i in 0..thread_count {
            threads.push(s.spawn(move || {
                for y in (i as i32..search_range).step_by(thread_count) {
                    let mut x = 0;

                    'x: while x < search_range {
                        for (s, range) in sensors.iter() {
                            if manhattan(*s, (x, y)) <= *range {
                                x = s.0 + (range - y.abs_diff(s.1)) as i32 + 1;
                                continue 'x;
                            }
                        }

                        return Some(x as usize * 4000000 + y as usize);
                    }
                }

                None
            }));
        }

        for thread in threads.into_iter() {
            if let Some(res) = thread.join().unwrap() {
                return res;
            }
        }

        0
    });

    (covered_count, tuning_freq)
}

pub fn solve(input: &str) -> (usize, usize) {
    solve_impl(input, 4000000)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let output = solve_impl(input, 20);
    assert_eq!(output.0, 26);
    assert_eq!(output.1, 56000011);
}
