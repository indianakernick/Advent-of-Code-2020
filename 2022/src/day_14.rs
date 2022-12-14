use std::collections::HashSet;
use text_io::scan;

fn draw_line(tiles: &mut HashSet<(u32, u32)>, a: (u32, u32), b: (u32, u32)) {
    if a.0 == b.0 {
        for y in a.1.min(b.1)..=a.1.max(b.1) {
            tiles.insert((a.0, y));
        }
    } else if a.1 == b.1 {
        for x in a.0.min(b.0)..=a.0.max(b.0) {
            tiles.insert((x, a.1));
        }
    } else {
        panic!("Diagonal line");
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut tiles = HashSet::<(u32, u32)>::new();
    let mut lowest_y = 0;

    for line in input.lines() {
        let mut line_bytes = line.bytes().peekable();

        let mut x1: u32;
        let mut y1: u32;
        let mut x2: u32;
        let mut y2: u32;

        scan!(line_bytes => "{},{} -> {},{}", x1, y1, x2, y2);
        draw_line(&mut tiles, (x1, y1), (x2, y2));

        lowest_y = lowest_y.max(y1);
        lowest_y = lowest_y.max(y2);

        while line_bytes.peek().is_some() {
            let x3: u32;
            let y3: u32;

            scan!(line_bytes => "-> {},{}", x3, y3);

            lowest_y = lowest_y.max(y3);

            (x1, y1) = (x2, y2);
            (x2, y2) = (x3, y3);

            draw_line(&mut tiles, (x1, y1), (x2, y2));
        }
    }

    let mut sound_count = 0;

    'outer: loop {
        let mut sand = (500, 0);

        loop {
            if sand.1 > lowest_y {
                break 'outer;
            }

            if !tiles.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
                continue;
            }

            if !tiles.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
                continue;
            }

            if !tiles.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
                continue;
            }

            tiles.insert(sand);
            sound_count += 1;
            break;
        }
    }

    (sound_count, 0)
}
