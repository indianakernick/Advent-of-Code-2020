use adventofcode2020::*;
use std::collections::HashSet;

fn main() {
    let mut every_tile = HashSet::<(i32, i32, i32)>::new();
    let mut set = HashSet::<(i32, i32, i32)>::new();

    lines_from_file("input/day_24.txt", |line| {
        let mut position = (0, 0, 0);
        let mut char_iter = line.chars();
        loop {
            let ch = match char_iter.next() {
                Some(ch) => ch,
                None => break
            };
            match ch {
                'e' => { position.0 += 1; position.1 -= 1; }
                'w' => { position.0 -= 1; position.1 += 1; }
                'n' => match char_iter.next().unwrap() {
                    'e' => { position.0 += 1; position.2 -= 1; }
                    'w' => { position.1 += 1; position.2 -= 1;}
                    _ => panic!()
                },
                's' => match char_iter.next().unwrap() {
                    'e' => { position.1 -= 1; position.2 += 1; }
                    'w' => { position.0 -= 1; position.2 += 1; }
                    _ => panic!()
                },
                _ => panic!()
            }
        }
        every_tile.insert(position);
        if set.contains(&position) {
            set.remove(&position);
        } else {
            set.insert(position);
        }
    });

    println!("Part one: {}", set.len());

    for _ in 0..100 {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;
        for tile in set.iter() {
            min_x = min_x.min(tile.0);
            max_x = max_x.max(tile.0);
            min_y = min_y.min(tile.1);
            max_y = max_y.max(tile.1);
            min_z = min_z.min(tile.2);
            max_z = max_z.max(tile.2);
        }

        let mut next_set = HashSet::<(i32, i32, i32)>::new();

        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    if x + y + z != 0 {
                        continue;
                    }
                    let tile = &(x, y, z);
                    let neighbors = [
                        (tile.0 + 1, tile.1 - 1, tile.2),
                        (tile.0 - 1, tile.1 + 1, tile.2),
                        (tile.0 + 1, tile.1,     tile.2 - 1),
                        (tile.0,     tile.1 + 1, tile.2 - 1),
                        (tile.0,     tile.1 - 1, tile.2 + 1),
                        (tile.0 - 1, tile.1,     tile.2 + 1),
                    ];
                    let mut black_count = 0;
                    for neighbor in neighbors.iter() {
                        if set.contains(neighbor) {
                            black_count += 1;
                        }
                    }
                    if set.contains(tile) {
                        if black_count == 1 || black_count == 2 {
                            next_set.insert(*tile);
                        }
                    } else {
                        if black_count == 2 {
                            next_set.insert(*tile);
                        }
                    }
                }
            }
        }

        set = next_set;
    }
    println!("Part two: {}", set.len());
}
