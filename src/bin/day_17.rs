use adventofcode2020::*;

/*
.###.#.#
####.#.#
#.....#.
####....
#...##.#
########
..#####.
######.#
*/

use std::collections::HashSet;

type Pos = (i32, i32, i32, i32);

type Cubes = HashSet<Pos>;

fn count_neighbors(cubes: &Cubes, pos: Pos) -> i32 {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    if cubes.contains(&(pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + w)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn simulate(cubes: &Cubes) -> Cubes {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;
    let mut min_w = i32::MAX;
    let mut max_w = i32::MIN;

    for pos in cubes.iter() {
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
        min_z = min_z.min(pos.2);
        max_z = max_z.max(pos.2);
        min_w = min_w.min(pos.3);
        max_w = max_w.max(pos.3);
    }

    let mut next_cubes = Cubes::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    let pos = (x, y, z, w);
                    let count = count_neighbors(&cubes, pos);
                    if cubes.contains(&pos) {
                        if count == 2 || count == 3 {
                            next_cubes.insert(pos);
                        }
                    } else {
                        if count == 3 {
                            next_cubes.insert(pos);
                        }
                    }
                }
            }
        }
    }
    next_cubes
}

fn main() {
    let mut cubes = Cubes::new();
    cubes.insert((1, 0, 0, 0));
    cubes.insert((2, 0, 0, 0));
    cubes.insert((3, 0, 0, 0));
    cubes.insert((5, 0, 0, 0));
    cubes.insert((7, 0, 0, 0));

    cubes.insert((0, 1, 0, 0));
    cubes.insert((1, 1, 0, 0));
    cubes.insert((2, 1, 0, 0));
    cubes.insert((3, 1, 0, 0));
    cubes.insert((5, 1, 0, 0));
    cubes.insert((7, 1, 0, 0));

    cubes.insert((0, 2, 0, 0));
    cubes.insert((6, 2, 0, 0));

    cubes.insert((0, 3, 0, 0));
    cubes.insert((1, 3, 0, 0));
    cubes.insert((2, 3, 0, 0));
    cubes.insert((3, 3, 0, 0));

    cubes.insert((0, 4, 0, 0));
    cubes.insert((4, 4, 0, 0));
    cubes.insert((5, 4, 0, 0));
    cubes.insert((7, 4, 0, 0));

    cubes.insert((0, 5, 0, 0));
    cubes.insert((1, 5, 0, 0));
    cubes.insert((2, 5, 0, 0));
    cubes.insert((3, 5, 0, 0));
    cubes.insert((4, 5, 0, 0));
    cubes.insert((5, 5, 0, 0));
    cubes.insert((6, 5, 0, 0));
    cubes.insert((7, 5, 0, 0));

    cubes.insert((2, 6, 0, 0));
    cubes.insert((3, 6, 0, 0));
    cubes.insert((4, 6, 0, 0));
    cubes.insert((5, 6, 0, 0));
    cubes.insert((6, 6, 0, 0));

    cubes.insert((0, 7, 0, 0));
    cubes.insert((1, 7, 0, 0));
    cubes.insert((2, 7, 0, 0));
    cubes.insert((3, 7, 0, 0));
    cubes.insert((4, 7, 0, 0));
    cubes.insert((5, 7, 0, 0));
    cubes.insert((7, 7, 0, 0));

    for _ in 0..6 {
        cubes = simulate(&cubes);
        println!("Count: {}", cubes.len());
    }
    println!("Part one: {}", cubes.len());
}
