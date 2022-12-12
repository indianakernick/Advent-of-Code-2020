pub fn solve(input: &str) -> (usize, usize) {
    let rows = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut visible_count = 2 * (rows.len() - 2) + 2 * rows[0].len();
    let mut max_score = 0;

    for y in 1..rows.len() - 1 {
        let vertical_edge = y == 0 || y == rows.len() - 1;

        for x in 1..rows[y].len() - 1 {
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

            visible_count += visible as usize;
            max_score = max_score.max(score);
        }
    }

    (visible_count, max_score)
}
