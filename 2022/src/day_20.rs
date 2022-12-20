pub fn solve(input: &str) -> (i32, usize) {
    let mut nums = Vec::<(usize, i32)>::new();

    for line in input.lines() {
        nums.push((nums.len(), line.parse().unwrap()));
    }

    for id in 0..nums.len() {
        let mut old_index = nums.iter().position(|(i, _)| *i == id).unwrap();
        let mut offset = nums[old_index].1;

        while offset > 0 {
            let len = nums.len();
            if old_index == len - 1 {
                let pair = nums.remove(len - 1);
                nums.insert(1, pair);
                old_index = 1;
            } else {
                nums.swap(old_index, old_index + 1);
                old_index += 1;
            }
            offset -= 1;
        }
        while offset < 0 {
            let len = nums.len();
            if old_index == 0 {
                let pair = nums.remove(0);
                nums.insert(len - 2, pair);
                old_index = len - 2;
            } else {
                nums.swap(old_index, old_index - 1);
                old_index -= 1;
                if old_index == 0 {
                    let pair = nums.remove(0);
                    nums.push(pair);
                    old_index = len - 1;
                }
            }
            offset += 1;
        }

        /*
        let new_index = (old_index + ((offset + 10 * nums.len() as i32) as usize)) % nums.len();
        if old_index as i32 + offset < 0 {
            nums.insert(new_index, nums[old_index]);
            nums.remove(old_index);
        } else {
            let pair = nums.remove(old_index);
            nums.insert(new_index, pair);
        }
        */
    }

    let zero_index = nums.iter().position(|(_, value)| *value == 0).unwrap();
    let first = nums[(zero_index + 1000) % nums.len()].1;
    let second = nums[(zero_index + 2000) % nums.len()].1;
    let third = nums[(zero_index + 3000) % nums.len()].1;

    (first + second + third, 0)
}
