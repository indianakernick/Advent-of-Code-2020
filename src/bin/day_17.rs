use adventofcode2020::*;

type Pos = (i32, i32, i32, i32);
type Cells = std::collections::HashSet<Pos>;

struct Range {
    min: i32,
    max: i32,
}

const EMPTY_RANGE: Range = Range {
    min: i32::MAX,
    max: i32::MIN,
};

impl Range {
    fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    fn iter(&self) -> std::ops::RangeInclusive<i32> {
        (self.min - 1)..=(self.max + 1)
    }

    fn add_to_range(&mut self, value: i32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
}

type Range3D = (Range, Range, Range);
type Range4D = (Range, Range, Range, Range);

fn add_to_range_3d(range: &mut Range3D, pos: Pos) {
    range.0.add_to_range(pos.0);
    range.1.add_to_range(pos.1);
    range.2.add_to_range(pos.2);
}

fn add_to_range_4d(range: &mut Range4D, pos: Pos) {
    range.0.add_to_range(pos.0);
    range.1.add_to_range(pos.1);
    range.2.add_to_range(pos.2);
    range.3.add_to_range(pos.3);
}

fn parse_input() -> Cells {
    let mut cells = Cells::new();
    let mut y = 0;

    lines_from_file("input/day_17.txt", |line| {
        let mut x = 0;
        for ch in line.chars() {
            if ch == '#' {
                cells.insert((x, y, 0, 0));
            }
            x += 1;
        }
        y += 1;
    });

    cells
}

fn count_neighbors_3d(cells: &Cells, pos: Pos) -> u32 {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                if cells.contains(&(pos.0 + x, pos.1 + y, pos.2 + z, 0)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_neighbors_4d(cells: &Cells, pos: Pos) -> u32 {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    if cells.contains(&(pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + w)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn simulate_3d(range: Range3D, next_cells: &mut Cells, cells: &Cells) -> Range3D {
    let mut next_range = (EMPTY_RANGE, EMPTY_RANGE, EMPTY_RANGE);

    for x in range.0.iter() {
        for y in range.1.iter() {
            for z in range.2.iter() {
                let pos = (x, y, z, 0);
                let count = count_neighbors_3d(&cells, pos);
                if count == 3 || (count == 2 && cells.contains(&pos)) {
                    next_cells.insert(pos);
                    add_to_range_3d(&mut next_range, pos);
                }
            }
        }
    }

    next_range
}

fn simulate_4d(range: Range4D, next_cells: &mut Cells, cells: &Cells) -> Range4D {
    let mut next_range = (EMPTY_RANGE, EMPTY_RANGE, EMPTY_RANGE, EMPTY_RANGE);

    for x in range.0.iter() {
        for y in range.1.iter() {
            for z in range.2.iter() {
                for w in range.3.iter() {
                    let pos = (x, y, z, w);
                    let count = count_neighbors_4d(&cells, pos);
                    if count == 3 || (count == 2 && cells.contains(&pos)) {
                        next_cells.insert(pos);
                        add_to_range_4d(&mut next_range, pos);
                    }
                }
            }
        }
    }

    next_range
}

fn part_one(mut cells: Cells) -> usize {
    let mut alt_cells = Cells::new();
    let mut range = (Range::new(0, 7), Range::new(0, 7), Range::new(0, 0));

    for _ in 0..6 {
        alt_cells.clear();
        range = simulate_3d(range, &mut alt_cells, &cells);
        std::mem::swap(&mut alt_cells, &mut cells);
    }

    cells.len()
}

fn part_two(mut cells: Cells) -> usize {
    let mut alt_cells = Cells::new();
    let mut range = (Range::new(0, 7), Range::new(0, 7), Range::new(0, 0), Range::new(0, 0));

    for _ in 0..6 {
        alt_cells.clear();
        range = simulate_4d(range, &mut alt_cells, &cells);
        std::mem::swap(&mut alt_cells, &mut cells);
    }

    cells.len()
}

fn main() {
    let cells = parse_input();
    println!("Part one: {}", part_one(cells.clone()));
    println!("Part two: {}", part_two(cells));
}
