use crate::common::{Grid, Dir, self};

pub fn solve(input: &str) -> (u32, u32) {
    const MAX_COUNT: u8 = 10;

    let grid = Grid::<u8>::from_input(input);
    let mut heat_loss = vec![u32::MAX; (grid.get_width() * grid.get_height() * 4 * MAX_COUNT as i32) as usize];
    let heat_loss_i = |((x, y), dir, count): ((i32, i32), Dir, u8)| {
        (y * grid.get_width() * 4 * MAX_COUNT as i32 + x * 4 * MAX_COUNT as i32 + (dir as i32) * MAX_COUNT as i32 + (count - 1) as i32) as usize
    };

    heat_loss[heat_loss_i(((0, 0), Dir::S, 1))] = 0;
    heat_loss[heat_loss_i(((0, 0), Dir::E, 1))] = 0;

    let mut unvisited = Vec::new();

    for x in 0..grid.get_width() {
        for y in 0..grid.get_height() {
            for dir in Dir::ALL {
                for dir_count in 1..=MAX_COUNT {
                    unvisited.push(((x, y), dir, dir_count));
                }
            }
        }
    }

    let mut neighbours = Vec::new();

    while unvisited.len() > 0 {
        let mut min_loss = u32::MAX;
        let mut min_index = usize::MAX;

        for i in 0..unvisited.len() {
            let loss = heat_loss[heat_loss_i(unvisited[i])];
            if loss < min_loss {
                min_loss = loss;
                min_index = i;
            }
        }

        if min_index == usize::MAX {
            break;
        }

        let (pos, prev_dir, dir_count) = unvisited.swap_remove(min_index);

        neighbours.clear();

        for next_dir in Dir::ALL {
            if next_dir == prev_dir.opposite() {
                continue;
            }

            let next_pos = common::add(pos, next_dir.to_vec());

            if !grid.valid(next_pos) {
                continue;
            }

            // if next_dir == prev_dir {
            //     if dir_count < 3 {
            //         neighbours.push((next_pos, next_dir, dir_count + 1));
            //     }
            //     continue;
            // }

            // neighbours.push((next_pos, next_dir, 1));

            if next_dir == prev_dir {
                if dir_count < 10 {
                    neighbours.push((next_pos, next_dir, dir_count + 1));
                }
                continue;
            }

            if dir_count >= 4 {
                neighbours.push((next_pos, next_dir, 1));
            }
        }

        if neighbours.is_empty() {
            break;
        }

        for neighbour in neighbours.iter() {
            let next_loss = (grid.get(neighbour.0) - b'0') as u32;
            let new_loss = min_loss + next_loss;
            let index = heat_loss_i(*neighbour);
            if new_loss < heat_loss[index] {
                heat_loss[index] = new_loss;
            }
        }
    }

    let mut min = u32::MAX;
    let end_pos = (grid.get_width() - 1, grid.get_height() - 1);

    for dir in Dir::ALL {
        for count in 1..=MAX_COUNT {
            min = min.min(heat_loss[heat_loss_i((end_pos, dir, count))]);
        }
    }

    (0, min)
}

#[cfg(test)]
#[test]
fn example_1() {
    let input =
"2413432311323
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
    // assert_eq!(output.0, 102);
    assert_eq!(output.1, 94);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input =
"111111111111
999999999991
999999999991
999999999991
999999999991";
    let output = solve(input);
    assert_eq!(output.1, 71);
}
