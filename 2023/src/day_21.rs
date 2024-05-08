use crate::common::Grid;

pub fn solve(input: &str) -> (usize, u64) {
    count_reachable(input, 64)
}

fn count_reachable(input: &str, steps: u32) -> (usize, u64) {
    let input_grid = Grid::<Tile>::from_input(&input);
    let stride = input_grid.get_stride();

    let mut read_vec = input_grid.to_vec();
    let mut write_vec = input_grid.to_vec();

    for _ in 0..steps {
        for idx in 0..read_vec.len() {
            if let Tile::Newline | Tile::Rock = write_vec[idx] {
                continue;
            }

            let sides = [idx.wrapping_sub(stride), idx + 1, idx + stride, idx.wrapping_sub(1)];
            let mut reachable = false;

            for side in sides {
                if side >= read_vec.len() { continue; }

                if let Tile::Start | Tile::Reachable = read_vec[side] {
                    reachable = true;
                    break;
                }
            }

            write_vec[idx] = if reachable { Tile::Reachable } else { Tile::Garden };
        }

        std::mem::swap(&mut read_vec, &mut write_vec);
    }

    let count = read_vec.iter().filter(|tile| **tile == Tile::Reachable).count();

    (count, 0)
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Newline = b'\n',
    Start = b'S',
    Garden = b'.',
    Rock = b'#',
    Reachable = b'O',
}

#[cfg(test)]
#[test]
fn example() {
    let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let output = count_reachable(input, 6);
    assert_eq!(output.0, 16);
}
