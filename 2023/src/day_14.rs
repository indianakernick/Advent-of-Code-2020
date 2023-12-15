pub fn solve(input: &str) -> (usize, u32) {
    let mut tiles = Vec::from(input.as_bytes());

    let width = tiles.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (tiles.len() + 1) / stride;

    for y in 0..height {
        let row_index = y * stride;

        for x in 0..width {
            let index = row_index + x;

            if tiles[index] == b'O' {
                push_north(&mut tiles, stride, index);
            }
        }
    }

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

    (sum, 0)
}

fn push_north(tiles: &mut [u8], stride: usize, mut index: usize) {
    while index > stride && tiles[index - stride] == b'.' {
        tiles[index - stride] = b'O';
        tiles[index] = b'.';
        index -= stride;
    }
}
