fn mix(nums: &mut Vec<(usize, i64)>) {
    let len = nums.len() as i64;

    for id in 0..nums.len() {
        let old_index = nums.iter().position(|(i, _)| *i == id).unwrap();
        let pair = nums[old_index];

        if pair.1 == 0 {
            continue;
        }

        let mut new_index = old_index as i64 + pair.1;

        if new_index <= 0 {
            new_index += (1 + (-new_index / (len - 1))) * (len - 1);
        }
        if new_index >= len {
            new_index -= (0 + (new_index / (len - 1))) * (len - 1);
        }

        let new_index = new_index as usize;

        if new_index == old_index {
            continue;
        }

        if new_index > old_index {
            nums.copy_within(old_index + 1..new_index + 1, old_index);
        } else if new_index < old_index {
            nums.copy_within(new_index..old_index, new_index + 1);
        }

        nums[new_index as usize] = pair;
    }
}

fn get_coord_sum(nums: &[(usize, i64)]) -> i64 {
    let zero_index = nums.iter().position(|(_, value)| *value == 0).unwrap();
    let first = nums[(zero_index + 1000) % nums.len()].1;
    let second = nums[(zero_index + 2000) % nums.len()].1;
    let third = nums[(zero_index + 3000) % nums.len()].1;
    first + second + third
}

pub fn solve(input: &str) -> (i64, i64) {
    let mut nums = Vec::<(usize, i64)>::new();

    for line in input.lines() {
        nums.push((nums.len(), line.parse().unwrap()));
    }

    let mut nums_2 = nums.clone();

    mix(&mut nums);

    for n in nums_2.iter_mut() {
        n.1 *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut nums_2);
    }

    (get_coord_sum(&nums), get_coord_sum(&nums_2))
}
