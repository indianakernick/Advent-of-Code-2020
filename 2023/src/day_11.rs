use std::collections::HashSet;

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut galaxies = HashSet::<(u32, u32)>::new();
    let mut lines = common::lines_iter(input).peekable();
    let first_line = lines.peek().unwrap();

    let width = first_line.len();
    let mut height = 0;
    let mut empty_rows = HashSet::<u32>::new();
    let mut empty_columns = (0..width as u32).collect::<HashSet<_>>();

    for line in lines {
        let mut empty_row = true;

        for (x, ch) in line.iter().enumerate() {
            if *ch == b'#' {
                let x = x as u32;
                empty_row = false;
                empty_columns.remove(&x);
                galaxies.insert((x, height));
            }
        }

        if empty_row {
            empty_rows.insert(height);
        }

        height += 1;
    }

    let mut sum = 0;

    for a in galaxies.iter() {
        for b in galaxies.iter() {
            let min_x = a.0.min(b.0);
            let max_x = a.0.max(b.0);
            let min_y = a.1.min(b.1);
            let max_y = a.1.max(b.1);

            let mut dist = (max_x - min_x) + (max_y - min_y);

            for x in min_x..max_x {
                if empty_columns.contains(&x) {
                    dist += 1;
                }
            }

            for y in min_y..max_y {
                if empty_rows.contains(&y) {
                    dist += 1;
                }
            }

            sum += dist;
        }
    }

    // We're visiting each pair twice.
    (sum / 2, 0)
}
