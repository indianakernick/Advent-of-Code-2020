use std::collections::HashSet;

use crate::common::{self, Dir, Grid};

pub fn solve(input: &str) -> (usize, usize) {
    let input_grid = Grid::<Tile>::from_input(&input);
    (count_reachable(input_grid, 64), count_reachable(input_grid, 26501365))
}

fn count_reachable(input_grid: Grid::<Tile>, steps: u32) -> usize {
    let mut read_set = HashSet::new();
    let mut write_set = HashSet::new();

    read_set.insert(input_grid.pos_of(Tile::Start).unwrap());

    for _ in 0..steps {
        for read_pos in read_set.iter() {
            for dir in Dir::ALL {
                let next = common::add(*read_pos, dir.to_vec());
                let mut normalised_next = next;

                if normalised_next.0 < 0 {
                    normalised_next.0 += input_grid.get_width()
                        * (1 + -normalised_next.0 / input_grid.get_width());
                }
                if normalised_next.1 < 0 {
                    normalised_next.1 += input_grid.get_height()
                        * (1 + -normalised_next.1 / input_grid.get_height());
                }

                normalised_next.0 %= input_grid.get_width();
                normalised_next.1 %= input_grid.get_height();

                if input_grid.get(normalised_next) != Tile::Rock {
                    write_set.insert(next);
                }
            }
        }

        std::mem::swap(&mut read_set, &mut write_set);
        write_set.clear();
    }

    read_set.len()
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
    let input_grid = Grid::<Tile>::from_input(&input);
    assert_eq!(count_reachable(input_grid, 6), 16);
    assert_eq!(count_reachable(input_grid, 10), 50);
    assert_eq!(count_reachable(input_grid, 50), 1594);
    assert_eq!(count_reachable(input_grid, 100), 6536);
    assert_eq!(count_reachable(input_grid, 500), 167004);
    assert_eq!(count_reachable(input_grid, 1000), 668697);
    assert_eq!(count_reachable(input_grid, 5000), 16733044);
}
