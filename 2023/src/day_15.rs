pub fn solve(input: &str) -> (u32, u32) {
    let bytes = input.as_bytes();
    let bytes = if bytes[bytes.len() - 1] == b'\n' {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    };

    let steps_hash_sum = bytes
        .split(|b| *b == b',')
        .map(|bytes| hash(bytes) as u32)
        .sum();

    let mut boxes: [Vec<(&[u8], u8)>; 256] = std::array::from_fn(|_| Vec::new());

    for step in bytes.split(|b| *b == b',') {
        let operation_index = step
            .iter()
            .position(|b| *b == b'=' || *b == b'-')
            .unwrap();
        let label = &step[..operation_index];
        let box_index = hash(label);
        let r#box = &mut boxes[box_index as usize];
        let lens_index = r#box.iter().position(|lens| lens.0 == label);

        if step[operation_index] == b'=' {
            let focal_length = step[operation_index + 1];
            if let Some(index) = lens_index {
                r#box[index].1 = focal_length;
            } else {
                r#box.push((label, focal_length));
            }
        } else {
            if let Some(index) = lens_index {
                r#box.remove(index);
            }
        }
    }

    let mut focusing_power_sum = 0;

    for box_index in 0..boxes.len() {
        let r#box = &boxes[box_index];
        for lens_index in 0..r#box.len() {
            focusing_power_sum += (box_index + 1) as u32
                * (lens_index + 1) as u32
                * (r#box[lens_index].1 - b'0') as u32;
        }
    }

    (steps_hash_sum, focusing_power_sum)
}

fn hash(bytes: &[u8]) -> u8 {
    let mut value = 0u8;

    for b in bytes {
        value = value.overflowing_add(*b).0.overflowing_mul(17).0;
    }

    value
}

#[cfg(test)]
#[test]
fn example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let output = solve(input);
    assert_eq!(output.0, 1320);
    assert_eq!(output.1, 145);
}
