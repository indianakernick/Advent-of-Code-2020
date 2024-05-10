use std::collections::HashSet;

use crate::common::{self, Dir, Grid};

pub fn solve(input: &str) -> (u32, usize) {
    let input_grid = Grid::<Tile>::from_input(&input);

    // (count_reachable(input_grid, 64), count_reachable(input_grid, 26501365))
    (count_reachable2(input_grid, 64), 0)
}

fn count_reachable2(input_grid: Grid<Tile>, steps: u32) -> u32 {
    let chunk_width = (input_grid.get_width() as usize + 2 + 63) / 64;
    let chunk_height = input_grid.get_height() as usize + 2;
    let mut mask = vec![0u64; chunk_width * chunk_height];

    for y in -1..input_grid.get_height() + 1 {
        let row_i = ((1 + y) as usize) * chunk_width;

        for x in -1..input_grid.get_width() + 1 {
            let block_i = row_i + ((1 + x) as usize) / 64;
            let tile_i = (1 + x) % 64;

            let tile = input_grid.get((
                (x + input_grid.get_width()) % input_grid.get_width(),
                (y + input_grid.get_height()) % input_grid.get_height(),
            ));

            if let Tile::Garden | Tile::Start = tile {
                mask[block_i] |= 1 << tile_i;
            }
        }
    }

    let mut read_set = vec![0u64; mask.len()];
    let mut write_set = vec![0u64; mask.len()];

    let start_pos = input_grid.pos_of(Tile::Start).unwrap();

    read_set[(1 + start_pos.1 as usize) * chunk_width + (1 + start_pos.0 as usize) / 64]
        |= 1 << ((1 + start_pos.0) % 64);

    for _ in 0..steps {
        for y in 1..chunk_height - 1 {
            let row_i = y * chunk_width;

            for x in 0..chunk_width {
                let block_i = row_i + x;
                let read_block = read_set[block_i];

                write_set[block_i - chunk_width] |= read_block; // north
                write_set[block_i + chunk_width] |= read_block; // south
                write_set[block_i] |= (read_block >> 1) | (read_block << 1); // west and east

                if x != 0 {
                    write_set[block_i - 1] |= read_block << 63; // west carry over
                }

                if x != chunk_width - 1 {
                    write_set[block_i + 1] |= read_block >> 63; // east carry over
                }
            }
        }

        for y in 0..chunk_height {
            let row_i = y * chunk_width;
            for x in 0..chunk_width {
                let block_i = row_i + x;
                write_set[block_i] &= mask[block_i];
            }
        }

        std::mem::swap(&mut read_set, &mut write_set);
        write_set.fill(0);
    }

    read_set.iter().map(|block| block.count_ones()).sum()
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
    assert_eq!(count_reachable2(input_grid, 1), 2);
    assert_eq!(count_reachable2(input_grid, 2), 4);
    assert_eq!(count_reachable2(input_grid, 3), 6);
    assert_eq!(count_reachable2(input_grid, 6), 16);
    // assert_eq!(count_reachable(input_grid, 10), 50);
    // assert_eq!(count_reachable(input_grid, 50), 1594);
    // assert_eq!(count_reachable(input_grid, 100), 6536);
    // assert_eq!(count_reachable(input_grid, 500), 167004);
    // assert_eq!(count_reachable(input_grid, 1000), 668697);
    // assert_eq!(count_reachable(input_grid, 5000), 16733044);
}
