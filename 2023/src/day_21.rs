use std::collections::{HashMap, HashSet};

use crate::common::{self, Dir, Grid};

pub fn solve(input: &str) -> (u32, u32) {
    let input_grid = Grid::<Tile>::from_input(&input);

    (count_reachable2(input_grid, 64), count_reachable2(input_grid, 26501365))
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

    let start_pos = input_grid.pos_of(Tile::Start).unwrap();
    let east_edge_mask = 1 << ((input_grid.get_width() + 1) % 64);

    read_set[(1 + start_pos.1 as usize) * chunk_width + (1 + start_pos.0 as usize) / 64]
        |= 1 << ((1 + start_pos.0) % 64);

    struct Chunk {
        read: Vec<u64>,
        write: Vec<u64>,
    }

    let mut chunks = HashMap::new();
    chunks.insert((0, 0), Chunk { read: read_set, write: vec![0u64; mask.len()] });

    struct BlockOperation {
        chunk_pos: (i32, i32),
        block_index: usize,
        block: u64,
    }

    let mut operations = Vec::new();

    for _ in 0..steps {
        // Move one step north, south, east and west from the read parts of the
        // chunks to the write parts of the chunks.

        for chunk in chunks.values_mut() {
            for y in 1..chunk_height - 1 {
                let row_i = y * chunk_width;

                for x in 0..chunk_width {
                    let block_i = row_i + x;
                    let read_block = chunk.read[block_i];

                    chunk.write[block_i - chunk_width] |= read_block; // north
                    chunk.write[block_i + chunk_width] |= read_block; // south
                    chunk.write[block_i] |= (read_block >> 1) | (read_block << 1); // west and east

                    if x != 0 {
                        chunk.write[block_i - 1] |= read_block << 63; // west carry over
                    }

                    if x != chunk_width - 1 {
                        chunk.write[block_i + 1] |= read_block >> 63; // east carry over
                    }
                }
            }
        }

        // Apply the mask to the write parts of each chunks.

        for chunk in chunks.values_mut() {
            for y in 0..chunk_height {
                let row_i = y * chunk_width;
                for x in 0..chunk_width {
                    let block_i = row_i + x;
                    chunk.write[block_i] &= mask[block_i];
                }
            }
        }

        // Check the edges of the chunks to find the new chunks that need to be
        // created.

        for (pos, chunk) in chunks.iter() {
            let mut vertical_check = |north: bool, pos: (i32, i32)| {
                let (from_offset, to_offset) = if north {
                    (0, (chunk_height - 2) * chunk_width)
                } else {
                    ((chunk_height - 1) * chunk_width, 1)
                };

                for x in 0..chunk_width {
                    let block = chunk.write[from_offset + x];

                    if block == 0 { continue; }

                    if x == 0 {
                        // west block
                        if block & 1 == 1 {
                            // vertical west corner
                            operations.push(BlockOperation {
                                chunk_pos: (pos.0 - 1, pos.1),
                                block_index: to_offset + chunk_width - 1,
                                block: east_edge_mask >> 1,
                            });
                        }
                        if block >> 1 != 0 {
                            // vertical side
                            operations.push(BlockOperation {
                                chunk_pos: pos,
                                block_index: to_offset + x,
                                block: block & !1,
                            });
                        }
                    }

                    if x == chunk_width - 1 {
                        // east block
                        if block & east_edge_mask != 0 {
                            // vertical east corner
                            operations.push(BlockOperation {
                                chunk_pos: (pos.0 + 1, pos.1),
                                block_index: to_offset,
                                block: 2,
                            });
                        }
                        if block & !east_edge_mask != 0 {
                            // vertical side
                            operations.push(BlockOperation {
                                chunk_pos: pos,
                                block_index: to_offset + x,
                                block: block & !east_edge_mask,
                            });
                        }
                    }

                    if x != 0 && x != chunk_width - 1 {
                        // vertical side
                        operations.push(BlockOperation {
                            chunk_pos: pos,
                            block_index: to_offset + x,
                            block,
                        });
                    }
                }
            };

            // north
            vertical_check(true, (pos.0, pos.1 - 1));
            // south
            vertical_check(false, (pos.0, pos.1 + 1));

            for y in 1..chunk_height - 1 {
                let west_block_index = y * chunk_width;
                let east_block_index = y * chunk_width + chunk_width - 1;

                // west
                if chunk.write[west_block_index] & 1 == 1 {
                    operations.push(BlockOperation {
                        chunk_pos: (pos.0 - 1, pos.1),
                        block_index: east_block_index,
                        block: east_edge_mask >> 1,
                    });
                }

                // east
                if chunk.write[east_block_index] & east_edge_mask != 0 {
                    operations.push(BlockOperation {
                        chunk_pos: (pos.0 + 1, pos.1),
                        block_index: west_block_index,
                        block: 2,
                    });
                }
            }
        }

        // Create the new chunks.

        for op in operations.iter() {
            let chunk = chunks
                .entry(op.chunk_pos)
                .or_insert_with(|| Chunk {
                    read: vec![0u64; mask.len()],
                    write: vec![0u64; mask.len()],
                });
            chunk.write[op.block_index] |= op.block;
        }

        operations.clear();

        // Swap the read and write.

        for chunk in chunks.values_mut() {
            std::mem::swap(&mut chunk.read, &mut chunk.write);
            chunk.write.fill(0);
        }
    }

    // Clear the edges.

    for chunk in chunks.values_mut() {
        for x in 0..chunk_width {
            // north
            chunk.read[x] = 0;
            // south
            chunk.read[(chunk_height - 1) * chunk_width + x] = 0;
        }
        for y in 1..chunk_height - 1 {
            // west
            chunk.read[y * chunk_width] &= !1;
            // east
            chunk.read[y * chunk_width + chunk_width - 1] &= !east_edge_mask;
        }
    }

    chunks
        .values()
        .map(|chunk| chunk.read.iter().map(|block| block.count_ones()).sum::<u32>())
        .sum()
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
    assert_eq!(count_reachable2(input_grid, 10), 50);
    assert_eq!(count_reachable2(input_grid, 50), 1594);
    assert_eq!(count_reachable2(input_grid, 100), 6536);
    assert_eq!(count_reachable2(input_grid, 500), 167004);
    assert_eq!(count_reachable2(input_grid, 1000), 668697);
    assert_eq!(count_reachable2(input_grid, 5000), 16733044);
}
