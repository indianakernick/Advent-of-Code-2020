use std::collections::HashSet;

use crate::common;

pub fn solve(input: &str) -> (u64, u64) {
    distance_sums(input, 1000000)
}

pub fn distance_sums(input: &str, empty_size: u64) -> (u64, u64) {
    let mut lines = common::lines_iter(input).peekable();
    let first_line = lines.peek().unwrap();

    let mut galaxies = Vec::<(u32, u32)>::new();
    let mut empty_rows = Vec::<u32>::new();
    let mut empty_columns = (0..first_line.len() as u32).collect::<HashSet<_>>();

    for (y, line) in lines.enumerate() {
        let mut empty_row = true;

        for (x, ch) in line.iter().enumerate() {
            if *ch == b'#' {
                let x = x as u32;
                empty_row = false;
                empty_columns.remove(&x);
                galaxies.push((x, y as u32));
            }
        }

        if empty_row {
            empty_rows.push(y as u32);
        }
    }

    let mut empty_columns = Vec::from_iter(empty_columns.iter().copied());

    empty_columns.sort_unstable();
    empty_rows.sort_unstable();

    let mut base_sum = 0;
    let mut empty_sum = 0;

    for a_i in 0..galaxies.len() {
        let a = galaxies[a_i];

        for b_i in a_i + 1..galaxies.len() {
            let b = galaxies[b_i];

            let min_x = a.0.min(b.0);
            let max_x = a.0.max(b.0);
            let min_y = a.1.min(b.1);
            let max_y = a.1.max(b.1);

            base_sum += (max_x - min_x) + (max_y - min_y);

            for x in empty_columns.iter() {
                if *x < min_x {
                    continue;
                }
                if *x > max_x {
                    break;
                }
                empty_sum += 1;
            }

            for y in empty_rows.iter() {
                if *y < min_y {
                    continue;
                }
                if *y > max_y {
                    break;
                }
                empty_sum += 1;
            }
        }
    }

    let base_sum = base_sum as u64;
    let empty_sum = empty_sum as u64;

    (base_sum + empty_sum, base_sum + (empty_size - 1) * empty_sum)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let output = distance_sums(input, 10);
    assert_eq!(output.0, 374);
    assert_eq!(output.1, 1030);
}
