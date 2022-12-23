use std::collections::{HashSet, HashMap};

fn bounding_area(elves: &HashSet<(i32, i32)>) -> i32 {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);

    for elf in elves.iter() {
        min = (min.0.min(elf.0), min.1.min(elf.1));
        max = (max.0.max(elf.0), max.1.max(elf.1));
    }

    (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as i32
}

#[derive(Clone, Copy)]
enum Dir8 {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

impl Dir8 {
    const ALL: [Dir8; 8] = [Dir8::NW, Dir8::N, Dir8::NE, Dir8::E, Dir8::SE, Dir8::S, Dir8::SW, Dir8::W];
    const NORTH: [Dir8; 3] = [Dir8::NW, Dir8::N, Dir8::NE];
    const EAST: [Dir8; 3] = [Dir8::NE, Dir8::E, Dir8::SE];
    const SOUTH: [Dir8; 3] = [Dir8::SE, Dir8::S, Dir8::SW];
    const WEST: [Dir8; 3] = [Dir8::SW, Dir8::W, Dir8::NW];
}

fn add(pos: (i32, i32), dir: Dir8) -> (i32, i32) {
    match dir {
        Dir8::NW => (pos.0 - 1, pos.1 - 1),
        Dir8::N => (pos.0, pos.1 - 1),
        Dir8::NE => (pos.0 + 1, pos.1 - 1),
        Dir8::E => (pos.0 + 1, pos.1),
        Dir8::SE => (pos.0 + 1, pos.1 + 1),
        Dir8::S => (pos.0, pos.1 + 1),
        Dir8::SW => (pos.0 - 1, pos.1 + 1),
        Dir8::W => (pos.0 - 1, pos.1),
    }
}

pub fn solve(input: &str) -> (i32, usize) {
    const DIRS: [Dir8; 4] = [
        Dir8::N, Dir8::S, Dir8::W, Dir8::E,
    ];
    const DIR_SETS: [&[Dir8; 3]; 4] = [
        &Dir8::NORTH, &Dir8::SOUTH, &Dir8::WEST, &Dir8::EAST,
    ];

    let mut elves = HashSet::<(i32, i32)>::new();
    let mut elves_buf = HashSet::<(i32, i32)>::new();
    let mut proposals = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();
    let mut dir_off = 0;
    let mut stable_round = 0;
    let mut round_10_area = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    for r in 1.. {
        let mut any_moved = false;

        for elf in elves.iter() {
            if Dir8::ALL.iter().map(|d| elves.contains(&add(*elf, *d))).all(|c| !c) {
                proposals.insert(*elf, vec![*elf]);
                continue;
            }

            let mut moved = false;

            for dir_idx in 0..4 {
                let dir = DIRS[(dir_idx + dir_off) % 4];
                let dir_set = DIR_SETS[(dir_idx + dir_off) % 4];

                if dir_set.iter().map(|d| elves.contains(&add(*elf, *d))).all(|c| !c) {
                    proposals.entry(add(*elf, dir))
                        .and_modify(|v| v.push(*elf))
                        .or_insert(vec![*elf]);
                    moved = true;
                    break;
                }
            }

            if !moved {
                proposals.insert(*elf, vec![*elf]);
            } else {
                any_moved = true;
            }
        }

        for (destination, sources) in proposals.iter() {
            if sources.len() > 1 {
                for elf in sources.iter() {
                    elves_buf.insert(*elf);
                }
            } else {
                elves_buf.insert(*destination);
            }
        }

        std::mem::swap(&mut elves, &mut elves_buf);
        elves_buf.clear();
        proposals.clear();
        dir_off += 1;

        if r == 10 {
            round_10_area = bounding_area(&elves);
        }

        if !any_moved {
            stable_round = r;
            break;
        }
    }

    (round_10_area, stable_round)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";
    let output = solve(input);
    assert_eq!(output.0, 110);
    assert_eq!(output.1, 20);
}
