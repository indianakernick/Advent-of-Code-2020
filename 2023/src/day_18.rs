use crate::common::{self, Dir};

pub fn solve(input: &str) -> (u64, u64) {
    let mut state_1 = State::default();
    let mut state_2 = State::default();

    for line in common::lines_iter(input) {
        let dir_1 = match line[0] {
            b'U' => Dir::N,
            b'R' => Dir::E,
            b'D' => Dir::S,
            b'L' => Dir::W,
            _ => panic!("Invalid input"),
        };

        let dist_1 = if line[3] == b' ' {
            line[2] - b'0'
        } else if line[4] == b' ' {
            (line[2] - b'0') * 10 + line[3] - b'0'
        } else {
            panic!("Invalid input");
        };

        let dir_2 = match line[line.len() - 2] {
            b'0' => Dir::E,
            b'1' => Dir::S,
            b'2' => Dir::W,
            b'3' => Dir::N,
            _ => panic!("Invalid input"),
        };

        let dist_2 = common::parse_u32_hex(&line[line.len() - 7..line.len() - 2]);

        state_1.reduce(dir_1, dist_1 as u32);
        state_2.reduce(dir_2, dist_2);
    }

    (state_1.area(), state_2.area())
}

#[derive(Default)]
struct State {
    position: (i32, i32),
    path_length: u32,
    interior_count: i64,
}

impl State {
    fn reduce(&mut self, dir: Dir, dist: u32) {
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let next_pos = common::add(self.position, dir.to_vec_mag(dist as i32));
        self.path_length += dist;
        self.interior_count += determinant(self.position, next_pos);
        self.position = next_pos;
    }

    fn area(self) -> u64 {
        // A variation of Pick's theorem.
        // https://en.wikipedia.org/wiki/Pick's_theorem
        let interior_count = self.interior_count + determinant(self.position, (0, 0));
        (interior_count.abs() as u64 + self.path_length as u64) / 2 + 1
    }
}

// Determinant of this matrix:
// | a.0 b.0 |
// | a.1 b.1 |
fn determinant(a: (i32, i32), b: (i32, i32)) -> i64 {
    a.0 as i64 * b.1 as i64 - b.0 as i64 * a.1 as i64
}

#[cfg(test)]
#[test]
fn example() {
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
    assert_eq!(output.1, 952408144115);
}
