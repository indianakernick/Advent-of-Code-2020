use std::collections::BinaryHeap;

use crate::common::{Grid, Dir, self};

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::<u8>::from_input(input);
    (search::<1>(&grid), search::<2>(&grid))
}

#[derive(PartialEq, Eq)]
struct Node {
    loss: u32,
    pos: (i32, i32),
    dir: Dir,
    count: u8,
}

impl Node {
    fn new(loss: u32, pos: (i32, i32), dir: Dir, count: u8) -> Node {
        Node { loss, pos, dir, count }
    }

    fn into_tuple(&self) -> ((i32, i32), Dir, u8) {
        (self.pos, self.dir, self.count)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.loss.cmp(&self.loss)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.dir.cmp(&other.dir))
            .then_with(|| self.count.cmp(&other.count))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn search<const PART: u8>(grid: &Grid) -> u32 {
    let min_count = if PART == 1 { 1 } else { 4 };
    let max_count = if PART == 1 { 3 } else { 10 };

    let mut heat_loss = vec![u32::MAX; (grid.get_width() * grid.get_height() * 4 * max_count) as usize];
    let heat_loss_i = |((x, y), dir, count): ((i32, i32), Dir, u8)| {
        debug_assert!(grid.valid((x, y)));
        debug_assert!(1 <= count && count <= max_count as u8);
        (
            y * grid.get_width() * 4 * max_count
            + x * 4 * max_count
            + (dir as i32) * max_count
            + (count - 1) as i32
        ) as usize
    };

    let mut queue = BinaryHeap::new();

    for dir in [Dir::E, Dir::S] {
        let vec = dir.to_vec();
        let loss = (grid.get(vec) - b'0') as u32;
        heat_loss[heat_loss_i((vec, dir, 1))] = loss;
        queue.push(Node::new(loss, vec, dir, 1));
    }

    let mut neighbours = Vec::new();

    while let Some(node) = queue.pop() {
        // Necessary because we're not replacing nodes on the queue, we're
        // pushing a duplicate when a better path is found for the same node.
        if node.loss > heat_loss[heat_loss_i(node.into_tuple())] {
            continue;
        }

        neighbours.clear();

        for next_dir in Dir::ALL {
            if next_dir == node.dir.opposite() {
                continue;
            }

            let next_pos = common::add(node.pos, next_dir.to_vec());

            if !grid.valid(next_pos) {
                continue;
            }

            if next_dir == node.dir {
                if node.count < max_count as u8 {
                    neighbours.push((next_pos, next_dir, node.count + 1));
                }
                continue;
            }

            // The count should never be 0. min_count == 1 when PART == 1.
            // Another way to help the compiler optimise this would be to use
            // NonZeroU8 but that makes the code a bit ugly.
            if PART == 1 || node.count >= min_count {
                neighbours.push((next_pos, next_dir, 1));
            }
        }

        for neighbour in neighbours.iter() {
            let neighbour_loss = (grid.get(neighbour.0) - b'0') as u32;
            let new_loss = node.loss + neighbour_loss;
            let index = heat_loss_i(*neighbour);
            if new_loss < heat_loss[index] {
                queue.push(Node::new(new_loss, neighbour.0, neighbour.1, neighbour.2));
                heat_loss[index] = new_loss;
            }
        }
    }

    let mut min = u32::MAX;
    let end_pos = (grid.get_width() - 1, grid.get_height() - 1);

    for dir in [Dir::E, Dir::S] {
        for count in min_count..=max_count as u8 {
            min = min.min(heat_loss[heat_loss_i((end_pos, dir, count))]);
        }
    }

    min
}

#[cfg(test)]
#[test]
fn example_1() {
    let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let output = solve(input);
    assert_eq!(output.0, 102);
    assert_eq!(output.1, 94);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input = "\
111111111111
999999999991
999999999991
999999999991
999999999991";
    let output = solve(input);
    assert_eq!(output.1, 71);
}
