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
}

type Range3D = (Range, Range, Range);
type Range4D = (Range, Range, Range, Range);

fn add_to_range_3d(range: &mut Range3D, pos: Pos) {
    range.0 = Range {
        min: range.0.min.min(pos.0),
        max: range.0.max.max(pos.0)
    };
    range.1 = Range {
        min: range.1.min.min(pos.1),
        max: range.1.max.max(pos.1)
    };
    range.2 = Range {
        min: range.2.min.min(pos.2),
        max: range.2.max.max(pos.2)
    };
}

fn add_to_range_4d(range: &mut Range4D, pos: Pos) {
    range.0 = Range {
        min: range.0.min.min(pos.0),
        max: range.0.max.max(pos.0)
    };
    range.1 = Range {
        min: range.1.min.min(pos.1),
        max: range.1.max.max(pos.1)
    };
    range.2 = Range {
        min: range.2.min.min(pos.2),
        max: range.2.max.max(pos.2)
    };
    range.3 = Range {
        min: range.3.min.min(pos.3),
        max: range.3.max.max(pos.3)
    };
}

fn parse_input() -> Cells {
    let mut cells = Cells::new();

    cells.insert((1, 0, 0, 0));
    cells.insert((2, 0, 0, 0));
    cells.insert((3, 0, 0, 0));
    cells.insert((5, 0, 0, 0));
    cells.insert((7, 0, 0, 0));

    cells.insert((0, 1, 0, 0));
    cells.insert((1, 1, 0, 0));
    cells.insert((2, 1, 0, 0));
    cells.insert((3, 1, 0, 0));
    cells.insert((5, 1, 0, 0));
    cells.insert((7, 1, 0, 0));

    cells.insert((0, 2, 0, 0));
    cells.insert((6, 2, 0, 0));

    cells.insert((0, 3, 0, 0));
    cells.insert((1, 3, 0, 0));
    cells.insert((2, 3, 0, 0));
    cells.insert((3, 3, 0, 0));

    cells.insert((0, 4, 0, 0));
    cells.insert((4, 4, 0, 0));
    cells.insert((5, 4, 0, 0));
    cells.insert((7, 4, 0, 0));

    cells.insert((0, 5, 0, 0));
    cells.insert((1, 5, 0, 0));
    cells.insert((2, 5, 0, 0));
    cells.insert((3, 5, 0, 0));
    cells.insert((4, 5, 0, 0));
    cells.insert((5, 5, 0, 0));
    cells.insert((6, 5, 0, 0));
    cells.insert((7, 5, 0, 0));

    cells.insert((2, 6, 0, 0));
    cells.insert((3, 6, 0, 0));
    cells.insert((4, 6, 0, 0));
    cells.insert((5, 6, 0, 0));
    cells.insert((6, 6, 0, 0));

    cells.insert((0, 7, 0, 0));
    cells.insert((1, 7, 0, 0));
    cells.insert((2, 7, 0, 0));
    cells.insert((3, 7, 0, 0));
    cells.insert((4, 7, 0, 0));
    cells.insert((5, 7, 0, 0));
    cells.insert((7, 7, 0, 0));

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

    range = simulate_3d(range, &mut alt_cells, &cells); // 1
    cells.clear();
    range = simulate_3d(range, &mut cells, &alt_cells); // 2
    alt_cells.clear();
    range = simulate_3d(range, &mut alt_cells, &cells); // 3
    cells.clear();
    range = simulate_3d(range, &mut cells, &alt_cells); // 4
    alt_cells.clear();
    range = simulate_3d(range, &mut alt_cells, &cells); // 5
    cells.clear();
    simulate_3d(range, &mut cells, &alt_cells); // 6

    cells.len()
}

fn part_two(mut cells: Cells) -> usize {
    let mut alt_cells = Cells::new();
    let mut range = (Range::new(0, 7), Range::new(0, 7), Range::new(0, 0), Range::new(0, 0));

    range = simulate_4d(range, &mut alt_cells, &cells); // 1
    cells.clear();
    range = simulate_4d(range, &mut cells, &alt_cells); // 2
    alt_cells.clear();
    range = simulate_4d(range, &mut alt_cells, &cells); // 3
    cells.clear();
    range = simulate_4d(range, &mut cells, &alt_cells); // 4
    alt_cells.clear();
    range = simulate_4d(range, &mut alt_cells, &cells); // 5
    cells.clear();
    simulate_4d(range, &mut cells, &alt_cells); // 6

    cells.len()
}

fn main() {
    let cells = parse_input();
    println!("Part one: {}", part_one(cells.clone()));
    println!("Part two: {}", part_two(cells));
}
