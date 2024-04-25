use crate::common::{self, Dir};

pub fn solve(input: &str) -> (u32, u32) {
    let instructions = common::lines_iter(input)
        .map(|line| {
            let dir = match line[0] {
                b'U' => Dir::N,
                b'R' => Dir::E,
                b'D' => Dir::S,
                b'L' => Dir::W,
                _ => panic!("Invalid input"),
            };

            let distance = if line[3] == b' ' {
                line[2] - b'0'
            } else if line[4] == b' ' {
                (line[2] - b'0') * 10 + line[3] - b'0'
            } else {
                panic!("Invalid input");
            };

            (dir, distance)
        })
        .collect::<Vec<_>>();

    let mut positions = Vec::with_capacity(instructions.len() + 1);
    let mut position = (0, 0);
    let mut path_length = 0;

    for (dir, distance) in instructions {
        positions.push(position);
        common::add_assign(&mut position, dir.to_vec_mag(distance as i32));
        path_length += distance as u32;
    }

    positions.push((0, 0));

    // Shoelace formula
    let interior_count = positions
        .windows(2)
        .map(|pair| pair[0].0 * pair[1].1 - pair[1].0 * pair[0].1)
        .sum::<i32>() / 2;

    // A slight modification of Pick's theorem.
    (interior_count.abs() as u32 + path_length / 2 + 1, 0)
}

#[cfg(test)]
#[test]
fn example_1() {
    let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    let output = solve(input);
    assert_eq!(output.0, 62);
}
