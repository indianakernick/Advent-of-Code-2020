use advent_of_code_2022 as util;

fn main() {
    let mut rows = Vec::new();

    util::each_line("input/day_08.txt", |line| {
        rows.push(line.as_bytes().to_vec());
    });

    let mut visible_count = 0;
    let mut max_score = 0;

    for y in 0..rows.len() {
        let vertical_edge = y == 0 || y == rows.len() - 1;

        for x in 0..rows[y].len() {
            let mut visible = vertical_edge || x == 0 || x == rows[y].len() - 1;
            let mut score = 1;
            let height = rows[y][x];

            for x1 in (0..x).rev() {
                if x1 == 0 && rows[y][x1] < height {
                    visible = true;
                }
                if x1 == 0 || rows[y][x1] >= height {
                    score *= x - x1;
                    break;
                }
            }

            for x1 in x + 1..rows[y].len() {
                if x1 == rows[y].len() - 1 && rows[y][x1] < height {
                    visible = true;
                }
                if x1 == rows[y].len() - 1 || rows[y][x1] >= height {
                    score *= x1 - x;
                    break;
                }
            }

            for y1 in (0..y).rev() {
                if y1 == 0 && rows[y1][x] < height {
                    visible = true;
                }
                if y1 == 0 || rows[y1][x] >= height {
                    score *= y - y1;
                    break;
                }
            }

            for y1 in y + 1..rows.len() {
                if y1 == rows.len() - 1 && rows[y1][x] < height {
                    visible = true;
                }
                if y1 == rows.len() - 1 || rows[y1][x] >= height {
                    score *= y1 - y;
                    break;
                }
            }

            if visible {
                visible_count += 1;
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 1: {}", visible_count);
    println!("Part 2: {}", max_score);
}
