pub fn solve(input: &str) -> (usize, usize) {
    let mut tiles = Vec::from(input.as_bytes());

    let width = tiles.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (tiles.len() + 1) / stride;

    tiles.resize(stride * (height - 1) + width, 0);

    let mut copies = vec![tiles.clone()];

    roll_north(&mut tiles, width, height, stride);

    let load_1 = north_load(&tiles, width, height, stride);

    roll_west(&mut tiles, width, height, stride);
    roll_south(&mut tiles, width, height, stride);
    roll_east(&mut tiles, width, height, stride);

    let index = loop {
        copies.push(tiles.clone());

        roll_north(&mut tiles, width, height, stride);
        roll_west(&mut tiles, width, height, stride);
        roll_south(&mut tiles, width, height, stride);
        roll_east(&mut tiles, width, height, stride);

        if let Some(index) = copies.iter().position(|copy| copy == &tiles) {
            break index;
        }
    };

    let tiles_index = index + (1000000000 - index) % (copies.len() - index);

    (load_1, north_load(&copies[tiles_index], width, height, stride))
}

fn roll_north(tiles: &mut [u8], width: usize, height: usize, stride: usize) {
    for y in 0..height {
        let row_index = y * stride;

        for x in 0..width {
            let index = row_index + x;

            if tiles[index] == b'O' {
                let mut index = index;
                while index > stride && tiles[index - stride] == b'.' {
                    tiles[index - stride] = b'O';
                    tiles[index] = b'.';
                    index -= stride;
                }
            }
        }
    }
}

fn roll_west(tiles: &mut [u8], width: usize, height: usize, stride: usize) {
    for x in 0..width {
        for y in 0..height {
            let index = y * stride + x;

            if tiles[index] == b'O' {
                let mut index = index;
                while index % stride > 0 && tiles[index - 1] == b'.' {
                    tiles[index - 1] = b'O';
                    tiles[index] = b'.';
                    index -= 1;
                }
            }
        }
    }
}

fn roll_south(tiles: &mut [u8], width: usize, height: usize, stride: usize) {
    for y in (0..height).rev() {
        let row_index = y * stride;

        for x in 0..width {
            let index = row_index + x;

            if tiles[index] == b'O' {
                let mut index = index;
                while index < tiles.len() - stride && tiles[index + stride] == b'.' {
                    tiles[index + stride] = b'O';
                    tiles[index] = b'.';
                    index += stride;
                }
            }
        }
    }
}

fn roll_east(tiles: &mut [u8], width: usize, height: usize, stride: usize) {
    for x in (0..width).rev() {
        for y in 0..height {
            let index = y * stride + x;

            if tiles[index] == b'O' {
                let mut index = index;
                while index % stride < width - 1 && tiles[index + 1] == b'.' {
                    tiles[index + 1] = b'O';
                    tiles[index] = b'.';
                    index += 1;
                }
            }
        }
    }
}

fn north_load(tiles: &[u8], width: usize, height: usize, stride: usize) -> usize {
    let mut sum = 0;

    for y in 0..height {
        let row_index = y * stride;

        for x in 0..width {
            let index = row_index + x;

            if tiles[index] == b'O' {
                sum += height - y;
            }
        }
    }

    sum
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let output = solve(input);
    assert_eq!(output.0, 136);
    assert_eq!(output.1, 64);
}
