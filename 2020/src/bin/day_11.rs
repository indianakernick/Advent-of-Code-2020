use adventofcode2020::*;

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    Vacant,
    Occupied
}

type TileVec = Vec::<Vec::<Tile>>;

fn occupied_or_null(tiles: &TileVec, x: usize, y: usize) -> bool {
    tiles
        .get(y)
        .and_then(|row| row.get(x))
        .map_or(false, |tile| *tile == Tile::Occupied)
}

fn direction_sum<F>(f: F) -> i32
    where F: Fn(usize, usize) -> i32
{
    let mut sum = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                sum += f(dx as usize, dy as usize)
            }
        }
    }
    sum
}

fn count_adjacent_occupied(tiles: &TileVec, x: usize, y: usize) -> i32 {
    direction_sum(|dx, dy| occupied_or_null(tiles, x.wrapping_add(dx), y.wrapping_add(dy)) as i32)
}

fn visible_occupied_or_null(tiles: &TileVec, mut x: usize, mut y: usize, dx: usize, dy: usize) -> bool {
    loop {
        x = x.wrapping_add(dx);
        y = y.wrapping_add(dy);
        match tiles.get(y as usize).and_then(|row| row.get(x as usize)) {
            None => return false,
            Some(Tile::Occupied) => return true,
            Some(Tile::Vacant) => return false,
            _ => {}
        }
    }
}

fn count_visible_occupied(tiles: &TileVec, x: usize, y: usize) -> i32 {
    direction_sum(|dx, dy| visible_occupied_or_null(tiles, x, y, dx, dy) as i32)
}

fn count_total_occupied(tiles: &TileVec) -> i32 {
    let height = tiles.len();
    let width = tiles[0].len();
    let mut total_occupied = 0;
    for y in 0..height {
        for x in 0..width {
            total_occupied += (tiles[y][x] == Tile::Occupied) as i32;
        }
    }
    total_occupied
}

fn find_equilibrium<F>(mut tiles: TileVec, threshold: i32, counter: F) -> i32
    where F: Fn(&TileVec, usize, usize) -> i32
{
    let mut next = tiles.clone();
    let height = tiles.len();
    let width = tiles[0].len();
    loop {
        let mut changed = false;
        for y in 0..height {
            for x in 0..width {
                let count = counter(&tiles, x, y);
                if count == 0 && tiles[y][x] == Tile::Vacant {
                    next[y][x] = Tile::Occupied;
                    changed = true;
                } else if count >= threshold && tiles[y][x] == Tile::Occupied {
                    next[y][x] = Tile::Vacant;
                    changed = true;
                } else {
                    next[y][x] = tiles[y][x];
                }
            }
        }
        if !changed {
            return count_total_occupied(&tiles);
        }
        tiles = next.clone();
    }
}

fn parse_input() -> TileVec {
    line_iter_from_file("input/day_11.txt")
        .map(|line| {
            line
                .bytes()
                .map(|byte| match byte {
                    b'.' => Tile::Floor,
                    b'L' => Tile::Vacant,
                    b'#' => Tile::Occupied,
                    _ => panic!()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let tiles = parse_input();
    println!("Part one: {}", find_equilibrium(tiles.clone(), 4, count_adjacent_occupied));
    println!("Part two: {}", find_equilibrium(tiles, 5, count_visible_occupied));
}
