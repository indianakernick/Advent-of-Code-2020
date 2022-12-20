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
            new_index += (1 + -new_index / (len - 1)) * (len - 1);
        }
        if new_index >= len {
            new_index -= new_index / (len - 1) * (len - 1);
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

        nums[new_index] = pair;
    }
}

fn get_coord_sum(nums: &[(usize, i64)]) -> i64 {
    let zero = nums.iter().position(|(_, value)| *value == 0).unwrap();
    [1000, 2000, 3000].iter().map(|off| nums[(zero + off) % nums.len()].1).sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    let mut nums_1 = Vec::<(usize, i64)>::new();

    for line in input.lines() {
        nums_1.push((nums_1.len(), line.parse().unwrap()));
    }

    let mut nums_2 = nums_1.clone();

    mix(&mut nums_1);

    for n in nums_2.iter_mut() {
        n.1 *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut nums_2);
    }

    (get_coord_sum(&nums_1), get_coord_sum(&nums_2))
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"1
2
-3
3
-2
0
4";
    let output = solve(input);
    assert_eq!(output.0, 3);
    assert_eq!(output.1, 1623178306);
}
