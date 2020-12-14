use adventofcode2020::*;

fn count_trees(lines: &Vec<String>, slope_x: usize, slope_y: usize) -> usize {
    let mut count = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < lines.len() {
        if lines[pos_y].chars().nth(pos_x) == Some('#') {
            count += 1;
        }
        pos_y += slope_y;
        pos_x = (pos_x + slope_x) % lines[0].len();
    }

    count
}

fn main() {
    let mut lines: Vec<String> = Vec::new();
    lines_from_file("input/day_3.txt", |line| {
        lines.push(line.clone());
    });

    println!("Part one: {}", count_trees(&lines, 3, 1));

    let mut product = 1;
    product *= count_trees(&lines, 1, 1);
    product *= count_trees(&lines, 3, 1);
    product *= count_trees(&lines, 5, 1);
    product *= count_trees(&lines, 7, 1);
    product *= count_trees(&lines, 1, 2);

    println!("Part two: {}", product);
}
