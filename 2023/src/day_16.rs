use std::collections::HashSet;

use crate::common;

pub fn solve(input: &str) -> (usize, u32) {
    let tiles = input.as_bytes();
    let width = tiles.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (tiles.len() + 1) / stride;

    let mut energised = HashSet::new();

    simulate(&mut energised, &tiles, stride, width, height, (0, 0), Dir::E);

    (energised.iter().map(|tile| tile.0).collect::<HashSet<_>>().len(), 0)
}

fn simulate(
    energised: &mut HashSet<((i32, i32), Dir)>,
    tiles: &[u8],
    stride: usize,
    width: usize,
    height: usize,
    position: (i32, i32),
    dir: Dir,
) {
    if position.0 < 0 || position.0 >= width as i32 || position.1 < 0 || position.1 >= height as i32 {
        return;
    }

    if !energised.insert((position, dir)) {
        return;
    }

    match (tiles[position.1 as usize * stride + position.0 as usize], dir) {
        (b'.', _) => simulate(energised, tiles, stride, width, height, common::add(position, dir.to_vec()), dir),
        (b'/', Dir::N) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::E.to_vec()), Dir::E),
        (b'/', Dir::E) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::N.to_vec()), Dir::N),
        (b'/', Dir::S) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::W.to_vec()), Dir::W),
        (b'/', Dir::W) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::S.to_vec()), Dir::S),
        (b'\\', Dir::N) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::W.to_vec()), Dir::W),
        (b'\\', Dir::E) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::S.to_vec()), Dir::S),
        (b'\\', Dir::S) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::E.to_vec()), Dir::E),
        (b'\\', Dir::W) => simulate(energised, tiles, stride, width, height, common::add(position, Dir::N.to_vec()), Dir::N),
        (b'|', Dir::N) | (b'|', Dir::S) => simulate(energised, tiles, stride, width, height, common::add(position, dir.to_vec()), dir),
        (b'|', Dir::E) | (b'|', Dir::W) => {
            simulate(energised, tiles, stride, width, height, common::add(position, Dir::N.to_vec()), Dir::N);
            simulate(energised, tiles, stride, width, height, common::add(position, Dir::S.to_vec()), Dir::S);
        }
        (b'-', Dir::N) | (b'-', Dir::S) => {
            simulate(energised, tiles, stride, width, height, common::add(position, Dir::E.to_vec()), Dir::E);
            simulate(energised, tiles, stride, width, height, common::add(position, Dir::W.to_vec()), Dir::W);
        }
        (b'-', Dir::W) | (b'-', Dir::E) => simulate(energised, tiles, stride, width, height, common::add(position, dir.to_vec()), dir),
        _ => panic!(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn to_vec(self) -> (i32, i32) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
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
}
