use adventofcode2020::*;
use std::collections::HashSet;

struct Range {
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
    min_z: i16,
    max_z: i16,
}

const EMPTY_RANGE: Range = Range {
    min_x: i16::MAX, max_x: i16::MIN,
    min_y: i16::MAX, max_y: i16::MIN,
    min_z: i16::MAX, max_z: i16::MIN,
};

impl Range {
    fn set(&mut self, tile: &(i16, i16, i16)) {
        self.min_x = self.min_x.min(tile.0);
        self.max_x = self.max_x.max(tile.0);
        self.min_y = self.min_y.min(tile.1);
        self.max_y = self.max_y.max(tile.1);
        self.min_z = self.min_z.min(tile.2);
        self.max_z = self.max_z.max(tile.2);
    }
}

fn parse_input() -> HashSet::<(i16, i16, i16)> {
    let mut tiles = HashSet::new();

    lines_from_file("input/day_24.txt", |line| {
        let mut tile = (0, 0, 0);
        let mut char_iter = line.chars();

        loop {
            let ch = match char_iter.next() {
                Some(ch) => ch,
                None => break
            };

            // https://www.redblobgames.com/grids/hexagons/
            match ch {
                'e' => { tile.0 += 1; tile.1 -= 1; }
                'w' => { tile.0 -= 1; tile.1 += 1; }
                'n' => match char_iter.next().unwrap() {
                    'e' => { tile.0 += 1; tile.2 -= 1; }
                    'w' => { tile.1 += 1; tile.2 -= 1; }
                    _ => panic!()
                },
                's' => match char_iter.next().unwrap() {
                    'e' => { tile.1 -= 1; tile.2 += 1; }
                    'w' => { tile.0 -= 1; tile.2 += 1; }
                    _ => panic!()
                },
                _ => panic!()
            }
        }

        if !tiles.remove(&tile) {
            tiles.insert(tile);
        }
    });

    tiles
}

fn simulate(tiles: &mut HashSet<(i16, i16, i16)>) {
    let mut next_tiles = HashSet::new();
    let mut range = EMPTY_RANGE;
    let mut next_range;

    for tile in tiles.iter() {
        range.set(tile);
    }

    for _ in 0..100 {
        next_tiles.clear();
        next_range = EMPTY_RANGE;

        for x in (range.min_x - 1)..=(range.max_x + 1) {
            for y in (range.min_y - 1)..=(range.max_y + 1) {
                for z in (range.min_z - 1)..=(range.max_z + 1) {
                    if x + y + z != 0 {
                        continue;
                    }
                    let neighbors = [
                        (x + 1, y - 1, z    ),
                        (x - 1, y + 1, z    ),
                        (x + 1, y,     z - 1),
                        (x,     y + 1, z - 1),
                        (x,     y - 1, z + 1),
                        (x - 1, y,     z + 1),
                    ];
                    let count = neighbors.iter()
                        .filter(|neighbor| tiles.contains(neighbor))
                        .count();
                    let tile = (x, y, z);
                    if count == 2 || (count == 1 && tiles.contains(&tile)) {
                        next_tiles.insert(tile);
                        next_range.set(&tile);
                    }
                }
            }
        }

        std::mem::swap(tiles, &mut next_tiles);
        range = next_range;
    }
}

fn main() {
    let mut tiles = parse_input();
    println!("Part one: {}", tiles.len());
    simulate(&mut tiles);
    println!("Part two: {}", tiles.len());
}
