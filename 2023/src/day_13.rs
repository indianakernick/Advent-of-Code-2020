pub fn solve(input: &str) -> (usize, usize) {
    let mut bytes = input.as_bytes();
    let mut sum = (0, 0);
    let mut tiles = Vec::new();

    loop {
        let start = bytes;
        let end = start
            .windows(2)
            .position(|pair| pair == b"\n\n")
            .unwrap_or(bytes.len());

        tiles.clear();
        tiles.extend_from_slice(&start[..end]);

        let width = tiles.iter().position(|b| *b == b'\n').unwrap();
        let stride = width + 1;
        let height = (end + 1) / stride;

        let first_mirror = find_mirror(&tiles, stride, width, height).unwrap();
        let second_mirror = find_alt_mirror(&mut tiles, stride, width, height, first_mirror).unwrap();

        sum.0 += first_mirror;
        sum.1 += second_mirror;

        if end + 2 >= bytes.len() {
            break;
        }
        bytes = &bytes[end + 2..];
    }

    sum
}

fn find_alt_mirror(
    tiles: &mut [u8],
    stride: usize,
    width: usize,
    height: usize,
    mirror: usize,
) -> Option<usize> {
    let skip = Some(mirror);

    for i in 0..tiles.len() {
        if tiles[i] == b'\n' {
            continue;
        }

        let old = tiles[i];

        tiles[i] = match tiles[i] {
            b'.' => b'#',
            b'#' => b'.',
            tile @ _ => tile,
        };

        if let Some(alt) = find_vertical_mirror(tiles, stride, width, height, skip) {
            if alt != mirror {
                tiles[i] = old;
                return Some(alt);
            }
        }

        if let Some(alt) = find_horizontal_mirror(tiles, stride, width, height, skip) {
            if alt != mirror {
                tiles[i] = old;
                return Some(alt);
            }
        }

        tiles[i] = old;
    }

    None
}

fn find_mirror(
    tiles: &[u8],
    stride: usize,
    width: usize,
    height: usize,
) -> Option<usize> {
    find_vertical_mirror(tiles, stride, width, height, None)
        .or_else(|| find_horizontal_mirror(tiles, stride, width, height, None))
}

fn find_vertical_mirror(
    tiles: &[u8],
    stride: usize,
    width: usize,
    height: usize,
    skip: Option<usize>,
) -> Option<usize> {
    'mirror_x: for mirror_x in 1..width {
        for y in 0..height {
            let row_index = y * stride;
            let left = &tiles[row_index..row_index + mirror_x];
            let right = &tiles[row_index + mirror_x..row_index + width];

            let (left, right) = if left.len() < right.len() {
                (left, &right[..left.len()])
            } else {
                (&left[left.len() - right.len()..], right)
            };

            if !left.iter().eq(right.iter().rev()) {
                continue 'mirror_x;
            }
        }

        if skip != Some(mirror_x) {
            return Some(mirror_x);
        }
    }

    None
}

fn find_horizontal_mirror(
    tiles: &[u8],
    stride: usize,
    width: usize,
    height: usize,
    skip: Option<usize>,
) -> Option<usize> {
    'mirror_y: for mirror_y in 1..height {
        for x in 0..width {
            let mirror_index = mirror_y * stride;

            let (top, bottom) = if mirror_y <= height / 2 {
                (
                    &tiles[x..x + mirror_index],
                    &tiles[x + mirror_index..x + 2 * mirror_index],
                )
            } else {
                let size = (height - mirror_y) * stride;
                (
                    &tiles[x + mirror_index - size..x + mirror_index - 1],
                    &tiles[x + mirror_index..],
                )
            };

            if !top.iter().step_by(stride).eq(bottom.iter().step_by(stride).rev()) {
                continue 'mirror_y;
            }
        }

        if skip != Some(mirror_y * 100) {
            return Some(mirror_y * 100);
        }
    }

    None
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let output = solve(input);
    assert_eq!(output.0, 405);
    assert_eq!(output.1, 400);
}
