use std::collections::HashSet;

use crate::common;

pub fn solve(input: &str) -> (usize, usize) {
    let tiles = input.as_bytes();
    let width = tiles.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (tiles.len() + 1) / stride;

    let mut energised = HashSet::new();

    simulate(&mut energised, &tiles, stride, width, height, (0, 0), Dir::E);

    let count_1 = energised_count(&energised);

    let mut count_2 = 0;

    assert_eq!(width, height);

    for pos in 0..width {
        energised.clear();
        simulate(&mut energised, &tiles, stride, width, height, (pos as i32, 0), Dir::S);
        let count = energised_count(&energised);
        count_2 = count_2.max(count);

        energised.clear();
        simulate(&mut energised, &tiles, stride, width, height, (width as i32 - 1, pos as i32), Dir::W);
        let count = energised_count(&energised);
        count_2 = count_2.max(count);

        energised.clear();
        simulate(&mut energised, &tiles, stride, width, height, (pos as i32, height as i32 - 1), Dir::N);
        let count = energised_count(&energised);
        count_2 = count_2.max(count);

        energised.clear();
        simulate(&mut energised, &tiles, stride, width, height, (0, pos as i32), Dir::E);
        let count = energised_count(&energised);
        count_2 = count_2.max(count);
    }

    (count_1, count_2)
}

fn energised_count(energised: &HashSet<((i32, i32), Dir)>) -> usize {
    energised.iter().map(|tile| tile.0).collect::<HashSet<_>>().len()
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
