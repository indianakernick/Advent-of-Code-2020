use std::collections::HashMap;

use crate::common::{self, Dir, Grid};

pub fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::from_input(input);
    let mut energised = HashMap::new();

    simulate(&mut energised, grid, (0, 0), Dir::E);

    let count_1 = energised.len();
    let mut count_2 = 0;
    let size = grid.get_width();

    assert_eq!(size, grid.get_height());

    for coord in 0..size {
        let sides = [
            (Dir::S, (coord, 0)),
            (Dir::W, (size - 1, coord)),
            (Dir::N, (coord, size - 1)),
            (Dir::E, (0, coord)),
        ];

        for (dir, pos) in sides {
            energised.clear();
            simulate(&mut energised, grid, pos, dir);
            count_2 = count_2.max(energised.len());
        }
    }

    (count_1, count_2)
}

fn simulate(
    energised: &mut HashMap<(i32, i32), u8>,
    grid: Grid,
    pos: (i32, i32),
    dir: Dir,
) {
    if !grid.valid(pos) {
        return;
    }

    let dir_bit = 1 << dir as u8;
    let mut exists = false;

    energised
        .entry(pos)
        .and_modify(|set| {
            if *set & dir_bit == dir_bit {
                exists = true;
            } else {
                *set |= dir_bit;
            }
        })
        .or_insert(dir_bit);

    if exists {
        return;
    }

    let next_dirs = match (grid.get(pos), dir) {
        (b'.', _) => (dir, None),
        (b'/', Dir::N) => (Dir::E, None),
        (b'/', Dir::E) => (Dir::N, None),
        (b'/', Dir::S) => (Dir::W, None),
        (b'/', Dir::W) => (Dir::S, None),
        (b'\\', Dir::N) => (Dir::W, None),
        (b'\\', Dir::E) => (Dir::S, None),
        (b'\\', Dir::S) => (Dir::E, None),
        (b'\\', Dir::W) => (Dir::N, None),
        (b'|', Dir::N) | (b'|', Dir::S) => (dir, None),
        (b'|', Dir::E) | (b'|', Dir::W) => (Dir::N, Some(Dir::S)),
        (b'-', Dir::N) | (b'-', Dir::S) => (Dir::E, Some(Dir::W)),
        (b'-', Dir::E) | (b'-', Dir::W) => (dir, None),
        _ => panic!(),
    };

    simulate(energised, grid, common::add(pos, next_dirs.0.to_vec()), next_dirs.0);

    if let Some(second) = next_dirs.1 {
        simulate(energised, grid, common::add(pos, second.to_vec()), second);
    }
}

#[cfg(test)]
#[test]
fn example() {
    let input =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let output = solve(input);
    assert_eq!(output.0, 46);
    assert_eq!(output.1, 51);
}
