pub fn solve(input: &str) -> (usize, u32) {
    let mut bytes = input.as_bytes();
    let mut sum = 0;

    loop {
        let start = bytes;
        let end = start
            .windows(2)
            .position(|pair| pair == b"\n\n")
            .unwrap_or(bytes.len());

        let width = start.iter().position(|b| *b == b'\n').unwrap();
        let stride = width + 1;
        let height = (end + 1) / stride;

        'checks: loop {
            'mirror_x: for mirror_x in 1..width {
                for y in 0..height {
                    let row_index = y * stride;
                    let left = &start[row_index..row_index + mirror_x];
                    let right = &start[row_index + mirror_x..row_index + width];

                    let (left, right) = if left.len() < right.len() {
                        (left, &right[..left.len()])
                    } else {
                        (&left[left.len() - right.len()..], right)
                    };

                    if !left.iter().eq(right.iter().rev()) {
                        continue 'mirror_x;
                    }
                }

                sum += mirror_x;
                break 'checks;
            }

            'mirror_y: for mirror_y in 1..height {
                for x in 0..width {
                    let mirror_index = mirror_y * stride;

                    let (top, bottom) = if mirror_y <= height / 2 {
                        (
                            &start[x..x + mirror_index],
                            &start[x + mirror_index..x + 2 * mirror_index]
                        )
                    } else {
                        let size = (height - mirror_y) * stride;
                        (
                            &start[x + mirror_index - size..x + mirror_index],
                            &start[x + mirror_index..x + mirror_index + size]
                        )
                    };

                    if !top.iter().step_by(stride).eq(bottom.iter().step_by(stride).rev()) {
                        continue 'mirror_y;
                    }
                }

                sum += 100 * mirror_y;
                break 'checks;
            }

            panic!("Unable to find line of symmetry");
        }

        if end + 2 >= bytes.len() {
            break;
        }
        bytes = &bytes[end + 2..];
    }

    (sum, 0)
}
